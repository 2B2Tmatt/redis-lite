use std::collections::HashMap;
use std::time::Instant;

pub enum Command {
    Get(String),
    Set(String, String),
    Del(String),
    Exists(String),
    Keys(String),
    Expire(String, i32),
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
                    return String::from("NULL");
                } else {
                    return String::from(data.value);
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
            Command::Del(k) => {
                let removed = self.map.remove(&k).is_some();
                if removed {
                    return String::from("1");
                } else {
                    return String::from("0");
                }
            }
            Command::Exists(k) => {
                let exists = self.map.contains_key(&k);
                if exists {
                    let expired = self.purge_if_expired(&k, Instant::now());
                    if expired {
                        return String::from("0");
                    }
                    return String::from("1");
                } else {
                    return String::from("0");
                }
            }
            Command::Keys(k) => {
                println!("not implemented yet");
                String::from("not yet")
            }
            Command::Expire(k, s) => String::from("not yet"),
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
                    return false;
                } else {
                    self.map.remove(key);
                    return true;
                }
            }
            None => {
                return false;
            }
        }
    }
}
