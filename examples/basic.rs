use hat_trie_cache::HatTrieCache;

fn main() {
    // Create a new HAT-trie cache with capacity of 100 entries
    let cache = HatTrieCache::new(100);
    
    // Insert some key-value pairs
    cache.insert("hello", "world");
    cache.insert("rust", "lang");
    
    // Retrieve values
    if let Some(value) = cache.get("hello") {
        println!("Found value for 'hello': {}", value);
    }
    
    // Check if a key exists
    println!("Contains 'rust': {}", cache.contains_key("rust"));
    println!("Contains 'python': {}", cache.contains_key("python"));
}
