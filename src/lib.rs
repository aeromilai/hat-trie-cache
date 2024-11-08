use std::sync::Arc;
use parking_lot::RwLock;
use quick_cache::sync::Cache;

pub struct HatTrieCache {
    cache: Arc<RwLock<Cache<String, String>>>,
}

impl HatTrieCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(Cache::new(capacity))),
        }
    }

    /// Creates a new HatTrieCache with default capacity of 1000 entries
    pub fn default() -> Self {
        Self::new(1000)
    }

    pub fn insert(&self, key: &str, value: &str) {
        self.cache.write().insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.cache.read().get(&key.to_string()).map(|s| s.clone())
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.cache.read().get(&key.to_string()).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let cache = HatTrieCache::default();
        
        cache.insert("test", "value");
        assert_eq!(cache.get("test"), Some("value".to_string()));
        assert!(cache.contains_key("test"));
        assert!(!cache.contains_key("nonexistent"));
    }
}
