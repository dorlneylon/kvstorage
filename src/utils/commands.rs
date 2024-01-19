#[derive(Debug)]
pub enum Command {
    Get(String),
    Set(String, String),
    Del(String),
    Incr(String, u64),
    Decr(String, u64),
    Transact(Vec<Command>),
    Rollback(String, u64),
    Clear(),
    Quit(),
    Help(),
    Unknown(String),
}

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
            "incr" => {
                let mut parts = s.splitn(3, ' ').skip(1);
                let key = parts.next().unwrap_or_default();
                let increment_value = parts.next().unwrap_or_default().parse::<u64>().unwrap();
                Command::Incr(key.to_string(), increment_value)
            },
            "decr" => {
                let mut parts = s.splitn(3, ' ').skip(1);
                let key = parts.next().unwrap_or_default();
                let increment_value = parts.next().unwrap_or_default().parse::<u64>().unwrap();
                Command::Decr(key.to_string(), increment_value)
            },
            "quit" => Command::Quit(),
            "transact:" => {
                let ss = s[s.find('\n').map(|i| i + 1).unwrap_or(s.len())..].to_string();
                let k = ss.replace("\r", "");
                let ops: Vec<_> = k.split("\n").map(Command::new).collect();
                Command::Transact(ops)
            },
            "help" => Command::Help(),
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
}