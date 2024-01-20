#[cfg(test)]
mod tests {
    use crate::storage::storage_manager::Storage;

    #[test]
    pub fn test_insertion() {
        let mut storage = Storage::new();
        storage.insert("key", "value").unwrap();
        assert_eq!(storage.get("key").unwrap(), "value".to_string());
    }

    #[test]
    pub fn test_deletion() {
        let mut storage = Storage::new();
        storage.insert("key", "value").unwrap();
        storage.delete("key").unwrap();
        assert!(storage.get("key").is_err());
    }

    #[test]
    pub fn test_version_control() {
        let mut storage = Storage::new();
        storage.insert("key", "value").unwrap();
        storage.insert("key", "value2").unwrap();
        assert_eq!(storage.val_from("key", 0).unwrap(), "value2".to_string());
        assert_eq!(storage.val_from("key", 1).unwrap(), "value".to_string());

        storage.delete("key").unwrap();
        assert!(storage.val_from("key", 0).is_err());
        assert!(storage.val_from("key", 1).is_ok_and(|x| x.eq("value2")));
        assert!(storage.val_from("key", 2).is_ok_and(|x| x.eq("value")));
    }
}