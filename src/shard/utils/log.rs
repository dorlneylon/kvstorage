use std::collections::HashMap;
use serde_json::to_string_pretty;
use std::io::{Write, Error, ErrorKind};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::common::commands::Command;
use std::fs::OpenOptions;

pub struct Logger;

impl Logger {
    pub fn log(command: &Command, respond: &Result<String, Error>, file: &str) -> Result<(), Error> {
        let mut file = OpenOptions::new().create(true).append(true).open(file)?;
        
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp = since_the_epoch.as_secs();

        let datetime = chrono::DateTime::<chrono::Utc>::from(start);
        let date_time_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        
        writeln!(file, "Time: {}, Date: {}", timestamp, date_time_str)?;
        writeln!(file, "Command: {:?}", command)?;
        match respond {
            Ok(response) => writeln!(file, "Response: {}", response)?,
            Err(e) => writeln!(file, "Error: {}", e)?,
        }

        match command {
            Command::Set(key, value) => {
                writeln!(file, "Set command - Key: {}, Value: {}", key, value)?;
            },
            Command::Get(key) => {
                writeln!(file, "Get command - Key: {}", key)?;
            },
            Command::Del(key) => {
                writeln!(file, "Del command - Key: {}", key)?;
            },
            Command::Incr(key, amount) => {
                writeln!(file, "Incr command - Key: {}, Amount: {}", key, amount)?;
            },
            Command::Decr(key, amount) => {
                writeln!(file, "Decr command - Key: {}, Amount: {}", key, amount)?;
            },
            Command::Rollback(key, commit) => {
                writeln!(file, "Rollback command - Key: {}, Commit: {}", key, commit)?;
            },
            _ => {},
        }

        write!(file, "\n")?;

        Ok(())
    }

    pub fn store(map: &HashMap<String, String>, file: &str) -> Result<(), Error> {
        let json = to_string_pretty(&map).map_err(|e| Error::new(ErrorKind::Other, e))?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file)?;

        file.write_all(json.as_bytes())?;
        file.flush()?;

        Ok(())
    }

    pub fn load(file: &str) -> Result<HashMap<String, String>, Error> {
        let mut _map = HashMap::new();
        let json = std::fs::read_to_string(file)?;
        _map = serde_json::from_str(&json)?;
        Ok(_map)
    }
}