use std::{time::Duration, collections::{BTreeSet, HashMap}};
use memcache::Client;
use crate::utils::errors::EitherError;

const H: usize = 200; // prolly should be moved to config.toml

pub struct CacheManager {
    pub instance: Option<Client>,
    pub last_mod: BTreeSet<(u64, String)>,
    pub query_mp: HashMap<String, u64>,   // auxiliary map to handle cache updates
    pub t: u64,
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
            query_mp: HashMap::new(),
            t: 0,
        }
    }

    fn process(&mut self, f: fn(&Client, &Vec<&str>) -> Result<String, EitherError>, args: &Vec<&str>) -> Result<String, EitherError> {
        match &self.instance {
            Some(client) => {
                let res = f(client, args);
                if args.len() > 0 {
                    self.query_mp.insert(args[0].to_string(), self.t);
                    self.last_mod.insert((self.t, args[0].to_string()));
                    self.t += 1;
                }
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
        
        let _unused = self.upd_cache();
        res
    }

    // todo! make the code look clean
    pub fn upd_cache(&mut self) -> Result<(), EitherError> {
        if self.last_mod.len() < H {
            return Ok(())
        }

        let half_point = self.last_mod.len() - H / 2;
        let to_process = self.last_mod.iter().take(H/2).cloned().collect::<Vec<_>>();
        for el in to_process.iter() {
            match self.query_mp.get(el.1.as_str()) {
                Some(val) => {
                    if *val == 1 {
                        let k = el.1.clone();
                        let _unused = self.process(
                            |client, args| {
                                client.set_read_timeout(Option::from(Duration::from_secs(1))).unwrap();
                                match client.delete(args[0]) {
                                    Ok(v) => Ok(v.to_string()),
                                    Err(e) => Err(EitherError::from_memcache(e)),
                                }
                            }, &vec![k.as_str()]
                        );
                        self.query_mp.remove(el.1.as_str());
                    } else {
                        self.query_mp.insert(el.1.clone(), *val - 1);
                    }
                },
                _ => continue,
            }
        }

        self.last_mod.split_off(&self.last_mod.iter().nth(half_point).unwrap().clone());

        Ok(())
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
            }, &vec![key]
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
            }, &vec![]);
        self.query_mp.clear();
        self.last_mod.clear();
        Ok(())
    }
}