use std::{collections::HashMap, io::Error};

#[derive(Clone)]
pub struct Storage {
    map: HashMap<String, String>,
    versions: Vec<HashMap<String, String>>, // todo! might be a better idea to implement persistent binary tree.
                                            // it won't be so efficient when it comes to queries (O(1) -> O(logn)), but
                                            // versions of hashmap probably take a lot more memory size.
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            map: HashMap::new(),
            versions: vec![HashMap::new()]
        }
    }

    pub fn new_from(mp: &HashMap<String, String>) -> Storage {
        Storage {
            map: mp.clone(),
            versions: vec![mp.clone()]
        }
    }

    pub fn get_map(&self) -> &HashMap<String, String> {
        &self.map
    }

    pub fn insert(&mut self, key: &str, val: &str) -> Result<String, Error> {
        self.map.insert(key.to_string(), val.to_string());
        self.versions.push(self.map.clone());
        Ok("OK".to_string())
    }

    pub fn get(&self, key: &str) -> Result<String, Error> {
        match self.map.get(key) {
            Some(val) => Ok(val.clone()),
            _ => Err(Error::new(std::io::ErrorKind::NotFound, "Item not found"))
        }
    }
    
    pub fn delete(&mut self, key: &str) -> Result<String, Error> {
        match self.map.remove(key) {
            Some(val) => {
                self.versions.push(self.map.clone());
                Ok(val)
            },
            _ => Err(Error::new(std::io::ErrorKind::NotFound, "Item not found"))
        }
    }

    /* 
        versions are indexed from newest to oldest
    */
    pub fn val_from(&self, key: &str, version: usize) -> Result<String, Error> {
        match self.versions.get(self.versions.len() - 1 - version) {
            Some(val) => match val.get(key) {
                Some(val) => Ok(val.clone()),
                _ => Err(Error::new(std::io::ErrorKind::NotFound, "Item not found"))
            },
            _ => Err(Error::new(std::io::ErrorKind::NotFound, "Item not found"))
        }
    }

    pub fn ver_from(&self, version: usize) -> Result<HashMap<String, String>, Error> {
        match self.versions.get(version) {
            Some(val) => Ok(val.clone()),
            _ => Err(Error::new(std::io::ErrorKind::NotFound, "Version ?"))
        }
    }

    pub fn flush(&mut self) -> Result<(), ()> {
        self.map.clear();
        self.versions.push(self.map.clone());
        Ok(())
    }

    pub fn get_cur_ver(&self) -> usize {
        self.versions.len()
    }
}