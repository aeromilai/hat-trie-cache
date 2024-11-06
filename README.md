# hat-trie-cache

A Rust implementation of a HAT-trie based cache.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
hat-trie-cache = "0.1"
```

### Example

```rust
let result = hat_trie_cache::add(40, 2);
assert_eq!(result, 42);
```

For more examples, see the [examples directory](examples/).

## Running Examples

```bash
cargo run --example basic
```
