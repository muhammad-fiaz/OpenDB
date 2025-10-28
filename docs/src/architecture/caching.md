# Caching Strategy

OpenDB uses an LRU (Least Recently Used) cache to accelerate reads while maintaining consistency.

## Cache Architecture

```
┌──────────────────────────────────┐
│         Application              │
└─────────────┬────────────────────┘
              │
         Read/Write
              │
┌─────────────▼────────────────────┐
│         LRU Cache                │
│  ┌──────┬──────┬──────┬──────┐  │
│  │ Hot1 │ Hot2 │ Hot3 │ Hot4 │  │
│  └──────┴──────┴──────┴──────┘  │
└─────────────┬────────────────────┘
              │
       Cache Miss/Write
              │
┌─────────────▼────────────────────┐
│      Storage Backend             │
│         (RocksDB)                │
└──────────────────────────────────┘
```

## Write-Through Policy

All writes go to storage **first**, then update the cache:

```rust
pub fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
    // 1. Write to storage (ensures durability)
    self.storage.put(ColumnFamilies::DEFAULT, key, value)?;
    
    // 2. Update cache
    self.cache.insert(key.to_vec(), value.to_vec());
    
    Ok(())
}
```

**Why Write-Through?**

- ✅ Durability: Data is persisted immediately
- ✅ Consistency: Cache never has uncommitted data
- ❌ Slower writes: Every write hits disk

**Alternative: Write-Back**

- ✅ Faster writes (batch to disk later)
- ❌ Risk of data loss if crash before flush
- ❌ More complex consistency model

## Cache Invalidation

Deletes remove from both cache and storage:

```rust
pub fn delete(&self, key: &[u8]) -> Result<()> {
    // 1. Delete from storage
    self.storage.delete(ColumnFamilies::DEFAULT, key)?;
    
    // 2. Invalidate cache
    self.cache.invalidate(&key.to_vec());
    
    Ok(())
}
```

## LRU Eviction

When cache reaches capacity, least-recently-used items are evicted:

```
Cache (capacity = 3):
  
Put("A", "1")  →  [A]
Put("B", "2")  →  [B, A]
Put("C", "3")  →  [C, B, A]
Get("A")       →  [A, C, B]  # A is now most recent
Put("D", "4")  →  [D, A, C]  # B evicted (LRU)
```

## Cache Sizes

Default cache sizes:

```rust
pub struct OpenDBOptions {
    pub kv_cache_size: usize,       // Default: 1000
    pub record_cache_size: usize,   // Default: 500
}
```

### Tuning Cache Size

```rust
let mut options = OpenDBOptions::default();
options.kv_cache_size = 10_000;      // More KV entries
options.record_cache_size = 2_000;   // More Memory records

let db = OpenDB::open_with_options("./db", options)?;
```

**Guidelines:**

- **Small cache (100-1000)**: Low memory, high cache miss rate
- **Medium cache (1000-10000)**: Balanced for most workloads
- **Large cache (10000+)**: High memory, low cache miss rate

## Cache Hit Rates

Monitor effectiveness (metrics to be added):

```
Hit Rate = Cache Hits / Total Reads
```

- **> 80%**: Excellent, cache is effective
- **50-80%**: Good, consider increasing size
- **< 50%**: Poor, increase cache or review access patterns

## Multi-Level Caching

OpenDB has two cache levels:

1. **Application Cache** (LRU): In-process, fast
2. **RocksDB Block Cache**: Built into RocksDB, shared

### RocksDB Block Cache

RocksDB has its own block cache (not exposed in current API):

```rust
// Future tuning option
opts.set_block_cache_size(256 * 1024 * 1024); // 256 MB
```

## Concurrent Access

Caches use `parking_lot::RwLock` for thread safety:

```rust
pub struct LruMemoryCache<K, V> {
    cache: RwLock<LruCache<K, V>>,
}
```

- **Reads**: Multiple concurrent readers
- **Writes**: Exclusive lock during insert/evict

## Cache Coherency Guarantees

1. **Write Visibility**: Writes are immediately visible after `put()` returns
2. **Delete Visibility**: Deletes are immediately visible after `delete()` returns
3. **Transaction Isolation**: Transactions bypass cache (read from storage snapshot)

## Best Practices

### Warm Up Cache

```rust
// Preload important data
let important_ids = vec!["mem_001", "mem_002", "mem_003"];
for id in important_ids {
    db.get_memory(id)?;  // Populate cache
}
```

### Avoid Thrashing

```rust
// ❌ Bad: Random access pattern, poor cache hit rate
for i in 0..1_000_000 {
    let random_key = generate_random_key();
    db.get(&random_key)?;
}

// ✅ Good: Sequential or localized access
for i in 0..1000 {
    db.get(&format!("key_{}", i).as_bytes())?;
}
```

### Cache Bypass for Large Scans

For scanning large datasets, consider bypassing cache (future feature):

```rust
// Future API
db.scan_prefix_no_cache(b"prefix")?;
```

## Next

- [Performance Tuning](../advanced/performance.md)
- [API Reference](../api/kv.md)
