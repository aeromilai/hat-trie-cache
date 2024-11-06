use std::sync::Arc;
use parking_lot::RwLock;
use quick_cache::sync::Cache;

pub struct HatTrieCache {
    cache: Arc<RwLock<Cache<String, String>>>,
}

impl HatTrieCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(Cache::new(1000))), // Default size of 1000 entries
        }
    }

    pub fn insert(&self, key: &str, value: &str) {
        self.cache.write().insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.cache.read().get(&key.to_string()).cloned()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.cache.read().contains_key(&key.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let cache = HatTrieCache::new();
        
        cache.insert("test", "value");
        assert_eq!(cache.get("test"), Some("value".to_string()));
        assert!(cache.contains_key("test"));
        assert!(!cache.contains_key("nonexistent"));
    }
}
