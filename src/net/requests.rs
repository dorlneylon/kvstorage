use std::io::{Error, ErrorKind};
use crate::{storage::storage_manager::Storage, cache::cache_manager::CacheManager, utils::{commands::Command, log::load}};

pub struct RequestHandler {
    cache: CacheManager,
    storage: Storage,
}

impl RequestHandler {
    pub fn new(cache_addr: &str) -> RequestHandler {
        RequestHandler {
            cache: CacheManager::new(cache_addr),
            storage: Storage::new(),
        }
    }

    pub fn new_from(cache_addr: &str, file: &str) -> RequestHandler {
        RequestHandler {
            cache: CacheManager::new(cache_addr),
            storage: Storage::new_from(&load(file).unwrap()),
        }
    }

    pub fn get_storage(&self) -> &Storage {
        &self.storage
    }

    /*
        the process is overall blatant: check if the value exists in LRU, fetch and change if yes, skip if no.
        after each operation we're pushing (key, value) to LRU.
    */
    pub fn process(&mut self, command: &Command) -> Result<String, Error> {
        match command {
            Command::Get(key) => {
                match self.cache.get(key) {
                    Ok(val) => Ok(val),
                    Err(e) => {
                        println!("{}", e.to_string());
                        match self.storage.get(key) {
                            Ok(val) => {
                                let _unused = self.cache.set(key, val.as_str());
                                Ok(val)
                            },
                            Err(e) => {
                                Err(e)
                            }
                        }
                    },
                }
            },
            Command::Set(key, val) => {
                match self.storage.insert(key, val) {
                    Ok(_) => match self.cache.set(key, val) {
                        Ok(s) => Ok(s),
                        Err(e) => {
                            println!("{}", e.to_string());
                            Ok("OK".to_string())
                        }
                    },
                    Err(e) => Err(e)
                }
            },
            Command::Incr(key, inc) => {
                let cur_val = self.storage.get(key);
                if let Err(error) = cur_val {
                    return Err(error);
                }
                let cur_val = cur_val.unwrap().parse::<i64>().map_err(|_| Error::new(std::io::ErrorKind::InvalidInput, "Value is not a valid i64"))?;
                match self.storage.insert(key, (cur_val + (*inc as i64)).to_string().as_str()) {
                    Ok(s) => {
                        match self.cache.get(key) {
                            Ok(_) => {
                                match self.cache.set(key, self.storage.get(key).unwrap().as_str()) {
                                    Ok(s) => Ok(s),
                                    Err(e) => Err(Error::new(std::io::ErrorKind::Other, e.to_string()))
                                }
                            },
                            Err(e) => {
                                println!("{}", e.to_string());
                                Ok(s)
                            }
                        }
                    },
                    Err(e) => Err(e)
                }
            },
            Command::Decr(key, dec) => {
                self.process(&Command::Incr(key.to_string(), !dec+1))
            },
            Command::Clear() => {
                if self.storage.flush().is_ok() && self.cache.flush().is_ok() {
                    Ok("OK".to_string())
                } else {
                    Err(Error::new(ErrorKind::Other, "something went wrong".to_string()))
                }
            },
            Command::Rollback(key, ver) => {
                self.storage.val_from(key, *ver as usize)
            },
            Command::Transact(s) => {
                let mut res = "".to_string();
                let start = self.storage.get_cur_ver();
                for op in s {
                    let cur = self.process(op);
                    if cur.is_err() {
                        self.storage = Storage::new_from(&self.storage.ver_from(start).unwrap());
                        let _unused = self.cache.flush();
                        return Err(Error::new(std::io::ErrorKind::Other, cur.unwrap_err().to_string()));
                    } else {
                        res += cur.unwrap().as_str();
                    }
                    res += "\n";
                }
                Ok(res.to_string())
            },
            Command::Del(key) => {
                match self.storage.delete(key) {
                    Ok(_) => {
                        match self.cache.del(key) {
                            Ok(s) => Ok(s),
                            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e.to_string()))
                        }
                    },
                    Err(e) => Err(e)
                }
            },
            _ => panic!("invalid command")
        }
    }
}
