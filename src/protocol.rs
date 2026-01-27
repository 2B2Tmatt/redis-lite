use crate::store::Command;

pub fn parse_line(line: &str) -> Result<Command, String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Err(String::from("empty command"));
    }

    match parts[0].to_uppercase().as_str() {
        "GET" if parts.len() == 2 => Ok(Command::Get(parts[1].to_string())),
        "SET" if parts.len() >= 3 => {
            let k = parts[1].to_string();
            let v = parts[2..].join(" ");
            Ok(Command::Set(k, v))
        }
        "DEL" if parts.len() == 2 => Ok(Command::Del(parts[1].to_string())),
        "EXISTS" if parts.len() == 2 => Ok(Command::Exists(parts[1].to_string())),
        "KEYS" if parts.len() == 2 => Ok(Command::Keys(parts[1].to_string())),
        "EXPIRE" if parts.len() == 3 => {
            let k = parts[1].to_string();
            let s = parts[2].parse::<i32>().map_err(|e| e.to_string())?;
            Ok(Command::Expire(k, s))
        }
        _ => Err(String::from(
            "usage: GET key | SET key value | DEL key | EXPIRE key seconds",
        )),
    }
}
