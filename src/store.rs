use std::collections::HashMap;
use std::time::{Duration, Instant};

pub enum Command {
    Get(String),
    Set(String, String),
    Setex(String, String, i32),
    Del(String),
    Exists(String),
    Keys(String),
    Expire(String, i32),
    Ttl(String),
    Ping(),
    Quit(),
}

#[derive(Clone, Debug)]
struct Data {
    value: String,
    expiration: Option<Instant>,
}
pub struct Store {
    map: HashMap<String, Data>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn apply(&mut self, cmd: Command) -> String {
        match cmd {
            Command::Get(k) => {
                let data = match self.get_if_alive(&k) {
                    Some(d) => d,
                    None => return String::from("NULL"),
                };

                data.value
            }
            Command::Set(k, v) => {
                self.map.insert(
                    k,
                    Data {
                        value: (v),
                        expiration: None,
                    },
                );
                String::from("OK")
            }
            Command::Setex(k, v, s) => {
                if s < 0 {
                    return String::from("(error) ERR invalid expire time in setex");
                }
                self.map.insert(
                    k,
                    Data {
                        value: (v),
                        expiration: Some(Instant::now() + Duration::new(s as u64, 0)),
                    },
                );
                String::from("OK")
            }
            Command::Del(k) => {
                let removed = self.map.remove(&k).is_some();
                if removed {
                    String::from("1")
                } else {
                    String::from("0")
                }
            }
            Command::Exists(k) => match self.get_if_alive(&k) {
                Some(_) => String::from("1"),
                None => String::from("0"),
            },
            Command::Keys(k) => {
                println!("not implemented yet: {k}");
                String::from("not yet")
            }
            Command::Expire(k, s) => {
                if s <= 0 {
                    self.map.remove(&k);
                    return String::from("0");
                }
                let data = match self.get_if_alive_mut(&k) {
                    Some(d) => d,
                    None => {
                        return String::from("0");
                    }
                };
                data.expiration = Some(Instant::now() + Duration::new(s as u64, 0));
                String::from("1")
            }
            Command::Ping() => String::from("PONG"),
            Command::Quit() => String::from("QUIT"),
            Command::Ttl(k) => {
                let data = match self.get_if_alive_mut(&k) {
                    Some(d) => d,
                    None => return String::from("-2"),
                };
                let exp = match data.expiration {
                    Some(e) => e,
                    None => {
                        return String::from("-1");
                    }
                };
                let time_from_expire = match exp.checked_duration_since(Instant::now()) {
                    Some(t) => t,
                    None => {
                        return String::from("-2");
                    }
                };

                time_from_expire.as_secs().to_string()
            }
        }
    }

    // read only
    fn get_if_alive(&mut self, key: &str) -> Option<Data> {
        let data = match self.map.get(key) {
            Some(data) => data.clone(),
            None => {
                return None;
            }
        };
        match data.expiration {
            Some(d) => {
                if d > Instant::now() {
                    Some(data)
                } else {
                    self.map.remove(key);
                    None
                }
            }
            None => Some(data),
        }
    }

    fn get_if_alive_mut(&mut self, key: &str) -> Option<&mut Data> {
        use std::collections::hash_map::Entry;
        let expired = match self.map.entry(key.to_string()) {
            Entry::Occupied(entry) => match entry.get().expiration {
                Some(exp) => exp <= Instant::now(),
                None => false,
            },
            Entry::Vacant(_) => {
                return None;
            }
        };

        if expired {
            self.map.remove(key);
            None
        } else {
            self.map.get_mut(key)
        }
    }
}
