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

pub enum Response {
    Simple(String),
    Bulk(String),
    Integer(i32),
    List(Vec<String>),
    Quit(),
    Error(String),
}

impl Store {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn apply(&mut self, cmd: Command) -> Response {
        match cmd {
            Command::Get(k) => {
                let data = match self.get_if_alive(&k) {
                    Some(d) => d,
                    None => return Response::Simple("NULL".to_string()),
                };

                Response::Bulk(data.value)
            }
            Command::Set(k, v) => {
                self.map.insert(
                    k,
                    Data {
                        value: (v),
                        expiration: None,
                    },
                );
                Response::Simple("OK".to_string())
            }
            Command::Setex(k, v, s) => {
                if s < 0 {
                    return Response::Error("(error) ERR invalid expire time in setex".to_string());
                }
                self.map.insert(
                    k,
                    Data {
                        value: (v),
                        expiration: Some(Instant::now() + Duration::new(s as u64, 0)),
                    },
                );
                Response::Simple("OK".to_string())
            }
            Command::Del(k) => {
                let removed = self.map.remove(&k).is_some();
                if removed {
                    Response::Integer(1)
                } else {
                    Response::Integer(0)
                }
            }
            Command::Exists(k) => match self.get_if_alive(&k) {
                Some(_) => Response::Integer(1),
                None => Response::Integer(0),
            },
            Command::Keys(k) => {
                let mut keys: Vec<String> = Vec::new();
                for key in self.map.keys() {
                    if Store::matches(&k, key) {
                        keys.push(key.clone());
                    }
                }
                Response::List(keys)
            }
            Command::Expire(k, s) => {
                if s <= 0 {
                    self.map.remove(&k);
                    return Response::Integer(0);
                }
                let data = match self.get_if_alive_mut(&k) {
                    Some(d) => d,
                    None => {
                        return Response::Integer(0);
                    }
                };
                data.expiration = Some(Instant::now() + Duration::new(s as u64, 0));
                Response::Integer(1)
            }
            Command::Ping() => Response::Simple("PONG".to_string()),
            Command::Quit() => Response::Quit(),
            Command::Ttl(k) => {
                let data = match self.get_if_alive_mut(&k) {
                    Some(d) => d,
                    None => return Response::Integer(-2),
                };
                let exp = match data.expiration {
                    Some(e) => e,
                    None => {
                        return Response::Integer(-1);
                    }
                };
                let time_from_expire = match exp.checked_duration_since(Instant::now()) {
                    Some(t) => t,
                    None => {
                        return Response::Integer(-2);
                    }
                };

                Response::Bulk(time_from_expire.as_secs().to_string())
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
    fn matches(pattern: &str, key: &str) -> bool {
        if pattern == "*" {
            return true;
        }

        let parts: Vec<&str> = pattern.split('*').collect();

        if parts.len() == 1 {
            return key == pattern;
        }

        if !key.starts_with(parts[0]) {
            return false;
        }
        if !key.ends_with(parts[parts.len() - 1]) {
            return false;
        }

        let mut pos = parts[0].len();
        for part in &parts[1..parts.len() - 1] {
            if let Some(found) = key[pos..].find(part) {
                pos += found + part.len();
            } else {
                return false;
            }
        }

        true
    }
}
