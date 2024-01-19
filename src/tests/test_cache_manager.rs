#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashMap};
    use memcache::Client;
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

    // IMPORTANT! set H = 6
    #[test]
    fn test_upd() {
        let mut btreeset: BTreeSet<(u64, String)> = BTreeSet::new();
        let mut hmap: HashMap<String, u64> = HashMap::new();

        btreeset.insert((0, "zxc".to_string()));
        btreeset.insert((1, "ahaha".to_string()));
        btreeset.insert((2, "666".to_string()));
        btreeset.insert((3, "kek".to_string()));
        btreeset.insert((4, "lol".to_string()));
        btreeset.insert((5, "zxc".to_string()));

        // expected: (3, kek), (4, lol), (5, zxc)

        hmap.insert("zxc".to_string(), 2);
        hmap.insert("ahaha".to_string(), 1);
        hmap.insert("666".to_string(), 1);
        hmap.insert("lol".to_string(), 1);
        hmap.insert("kek".to_string(), 1);

        let mut cache = CacheManager {
            instance: match Client::connect("memcache://localhost:11211?timeout=10&tcp_nodelay=true") {
                Ok(client) => Option::from(client),
                _ => {
                    println!("Failed to connect to memcached");
                    None
                }
            },
            last_mod: btreeset,
            query_mp: hmap,
            t: 6,
        };

        let _r = cache.upd_cache();
        assert_eq!(cache.query_mp.get("kek").unwrap(), &1);
        assert_eq!(cache.query_mp.get("ahaha").is_none(), true);
        assert_eq!(cache.query_mp.get("zxc").unwrap(), &1);

    }
}