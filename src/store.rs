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
    deadline: Option<Instant>,
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
                let data = match self.map.get(&k) {
                    Some(data) => data.clone(),
                    None => return String::from("NULL"),
                };
                let expired = self.purge_if_expired(&k, Instant::now());
                if expired {
                    String::from("NULL")
                } else {
                    data.value
                }
            }
            Command::Set(k, v) => {
                self.map.insert(
                    k,
                    Data {
                        value: (v),
                        deadline: None,
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
                        deadline: Some(Instant::now() + Duration::new(s as u64, 0)),
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
            Command::Exists(k) => {
                let exists = self.map.contains_key(&k);
                if exists {
                    let expired = self.purge_if_expired(&k, Instant::now());
                    if expired {
                        return String::from("0");
                    }
                    String::from("1")
                } else {
                    String::from("0")
                }
            }
            Command::Keys(k) => {
                println!("not implemented yet: {k}");
                String::from("not yet")
            }
            Command::Expire(k, s) => {
                let data = match self.map.get_mut(&k) {
                    Some(data) => data,
                    None => {
                        return String::from("0");
                    }
                };
                data.deadline = Some(Instant::now() + Duration::new(s as u64, 0));
                let expired = self.purge_if_expired(&k, Instant::now());
                if expired {
                    return String::from("0");
                }
                String::from("1")
            }
            Command::Ping() => String::from("PONG"),
            Command::Quit() => String::from("QUIT"),
            Command::Ttl(k) => {
                let data = match self.map.get(&k) {
                    Some(data) => data.clone(),
                    None => {
                        return String::from("2");
                    }
                };
                let deadline = match data.deadline {
                    Some(d) => d,
                    None => {
                        return String::from("-1");
                    }
                };
                let now = Instant::now();
                let expired = self.purge_if_expired(&k, now);
                if expired {
                    return String::from("-2");
                }
                let time_from_expire = match deadline.checked_duration_since(now) {
                    Some(t) => t,
                    None => {
                        return String::from("-2");
                    }
                };

                time_from_expire.as_secs().to_string()
            }
        }
    }

    fn purge_if_expired(&mut self, key: &str, now: Instant) -> bool {
        let data = match self.map.get(key) {
            Some(data) => data.clone(),
            None => {
                return false;
            }
        };
        match data.deadline {
            Some(d) => {
                if d > now {
                    false
                } else {
                    self.map.remove(key);
                    true
                }
            }
            None => false,
        }
    }
}
