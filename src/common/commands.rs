use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Command {
    Get(String),
    Set(String, String),
    Del(String),
    Incr(String, i64),
    Decr(String, i64),
    Transact(Vec<Command>),
    Rollback(String, u64),
    Clear(),
    Unknown(String),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(dead_code)]
impl Command {
    pub fn new(s: &str) -> Command {
        let command_str = s.split_whitespace().next().unwrap_or_default();
        match command_str.to_lowercase().as_str() {
            "get" => Command::Get(s[s.find(' ').map(|i| i + 1).unwrap_or(s.len())..].to_string()),
            "set" => {
                let mut parts = s.splitn(3, ' ').skip(1);
                let key = parts.next().unwrap_or_default();
                let value = parts.next().unwrap_or_default();
                Command::Set(key.to_string(), value.to_string())
            },
            "del" => Command::Del(s[s.find(' ').map(|i| i + 1).unwrap_or(s.len())..].to_string()),
            "incr" | "decr" => {
                let mut parts = s.splitn(3, ' ').skip(1);
                let key = parts.next().unwrap_or_default();
                let increment_value = parts.next().unwrap_or_default().parse::<i64>().unwrap();
                match command_str.to_lowercase().as_str() {
                    "incr" => Command::Incr(key.to_string(), increment_value),
                    "decr" => Command::Decr(key.to_string(), increment_value),
                    _ => Command::Unknown(s.to_string()),
                }
            },
            "transact:" => {
                let ss = s[s.find('\n').map(|i| i + 1).unwrap_or(s.len())..].to_string();
                let k = ss.replace("\r", "");
                let ops: Vec<_> = k.split("\n").map(Command::new).collect();
                Command::Transact(ops)
            },
            "rollback" => {
                let mut parts = s.splitn(3, ' ').skip(1);
                let key = parts.next().unwrap_or_default();
                let commit = parts.next().unwrap_or_default().parse::<u64>().unwrap();
                Command::Rollback(key.to_string(), commit)
            },
            "clear" => Command::Clear(),
            _ => Command::Unknown(s.to_string()),
        }
    }

    pub fn get_key(&self) -> String {
        match self {
            Command::Get(key) => key.clone(),
            Command::Set(key, _) => key.clone(),
            Command::Del(key) => key.clone(),
            Command::Incr(key, _) => key.clone(),
            Command::Decr(key, _) => key.clone(),
            Command::Transact(_) => String::new(),
            Command::Rollback(key, _) => key.clone(),
            Command::Clear() => String::new(),
            Command::Unknown(_) => String::new(),
        }
    }
}