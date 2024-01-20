use std::hash::Hash;
use std::hash::Hasher;
use std::io::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use crate::common::commands::Command;

pub struct CommandDistributor {
    pub num_shards: u32,
    pub offset: u32,
}

impl CommandDistributor {
    pub fn new(num_shards: u32, offset: u32) -> CommandDistributor {
        CommandDistributor {
            num_shards,
            offset,
        }
    }

    pub fn which(&self, key: String) -> u32 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as u32 % self.num_shards
    }

    pub async fn serialize_cmd(&self, cmd: Command) -> Result<String, Error> {
        let serialized_cmd = serde_json::to_string(&cmd)?;
        Ok(serialized_cmd)
    }
    
    pub async fn map_command(&self, stream: &mut TcpStream) -> Result<(Command, String), Error> {
        let mut buffer = vec![0; 1024];
        stream.read(&mut buffer).await?;
    
        println!("Request:\n{}\n\n", String::from_utf8_lossy(&buffer[..]));
        let request_str = String::from_utf8_lossy(&buffer[..]);
        let mut body = "";
        if let Some(pos) = request_str.find("\r\n\r\n") {
            let end = request_str.find('\0').unwrap_or(request_str.len());
            body = &request_str[pos + 4..end];
        }
        println!("Body:\n{}\n", body);
        
        let cmd = Command::new(body);
        let h = self.which(cmd.get_key());
        Ok((cmd, format!("http://127.0.0.1:{}", self.offset+h)))
    }
}