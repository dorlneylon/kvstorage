#[cfg(test)]
mod tests {
    use crate::cache::cache_manager::CacheManager;
    
    #[test]
    fn test_connection() {
        let cache = CacheManager::new("memcache://localhost:11211?timeout=1&tcp_nodelay=true");
        assert!(cache.instance.is_some());

        let cache = CacheManager::new("memcache://127.0.0.1:11311?timeout=1&tcp_nodelay=true");
        assert!(cache.instance.is_none());
    }

    #[test]
    fn test_setter() {
        let mut cache = CacheManager::new("memcache://localhost:11211?timeout=10&tcp_nodelay=true");
        let res1 = cache.set("key", "value");
        let res2 = cache.get("key");
        assert_eq!(res1.unwrap(), "OK".to_string());
        assert_eq!(res2.unwrap(), "value".to_string());
    }

    #[test]
    fn test_deleter() {
        let mut cache = CacheManager::new("memcache://localhost:11211?timeout=10&tcp_nodelay=true");
        let res1 = cache.del("ZXCZXC");
        assert_eq!(res1.unwrap(), "false".to_string());
        let res2 = cache.set("ZXCZXC", "value");
        assert_eq!(res2.unwrap(), "OK".to_string());
        let res3 = cache.del("ZXCZXC");
        assert_eq!(res3.unwrap(), "true".to_string());
    }
}