use std::{time::Duration, collections::BTreeSet};
use memcache::Client;
use crate::utils::errors::EitherError;

pub struct CacheManager {
    pub instance: Option<Client>,
    pub last_mod: BTreeSet<(u64, String)>,
}

impl CacheManager {
    pub fn new(addr: &str) -> CacheManager {
        CacheManager {
            instance: match Client::connect(addr) {
                Ok(client) => Option::from(client),
                _ => {
                    println!("Failed to connect to memcached");
                    None
                }
            },
            last_mod: BTreeSet::new(),
        }
    }

    fn process(&mut self, f: fn(&Client, &Vec<&str>) -> Result<String, EitherError>, args: &Vec<&str>) -> Result<String, EitherError> {
        match &self.instance {
            Some(client) => {
                let res = f(client, args);
                res
            },
            _ => Err(EitherError::from_str("Failed to connect to memcached")),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<String, EitherError> {
        let res = self.process(
            |client, args| {
                client.set_read_timeout(Option::from(Duration::from_secs(1))).unwrap();
                match client.set(args[0], args[1], 0) {
                    Ok(_) => Ok("OK".to_string()),
                    Err(e) => Err(EitherError::from_memcache(e)),
                }
            },
            &vec![key, value]
        );
        
        res
    }

    pub fn get(&mut self, key: &str) -> Result<String, EitherError> {
        self.process(
            |client, args| {
                client.set_read_timeout(Option::from(Duration::from_secs(1))).unwrap();
                match client.get::<String>(args[0]) {
                    Ok(Some(value)) => Ok(value),
                    Ok(None) => Ok("None".to_string()),
                    Err(e) => Err(EitherError::from_memcache(e)),
                }
            },
            &vec![key]
        )
    }

    pub fn del(&mut self, key: &str) -> Result<String, EitherError> {
        self.process(
            |client, args| {
                client.set_read_timeout(Option::from(Duration::from_secs(1))).unwrap();
                match client.delete(args[0]) {
                    Ok(v) => Ok(v.to_string()),
                    Err(e) => Err(EitherError::from_memcache(e)),
                }
            },
            &vec![key]
        )
    }

    pub fn flush(&mut self) -> Result<(), ()> {
        let _res = self.process(
            |client, _args| {
                client.set_read_timeout(Option::from(Duration::from_secs(1))).unwrap();
                match client.flush() {
                    Ok(_) => Ok("OK".to_string()),
                    _ => Err(EitherError::from_str("Something went wrong"))
                }
            },
            &vec![]
        );
        Ok(())
    }
}