# Extending OpenDB

OpenDB is designed to be extensible. This guide covers custom backends, plugins, and extensions.

## Custom Storage Backends

OpenDB uses the `StorageBackend` trait for pluggability.

### Storage Trait

```rust
pub trait StorageBackend: Send + Sync {
    fn get(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>>;
    fn put(&self, cf: &str, key: &[u8], value: &[u8]) -> Result<()>;
    fn delete(&self, cf: &str, key: &[u8]) -> Result<()>;
    fn exists(&self, cf: &str, key: &[u8]) -> Result<bool>;
    fn scan_prefix(&self, cf: &str, prefix: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>>;
    fn begin_transaction(&self) -> Result<Box<dyn Transaction>>;
    fn flush(&self) -> Result<()>;
    fn snapshot(&self) -> Result<Box<dyn Snapshot>>;
}
```

### Example: In-Memory Backend

```rust
use std::collections::HashMap;
use std::sync::RwLock;
use opendb::storage::{StorageBackend, Transaction, Snapshot};
use opendb::{Result, Error};

pub struct MemoryBackend {
    data: RwLock<HashMap<String, HashMap<Vec<u8>, Vec<u8>>>>,
}

impl MemoryBackend {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }
}

impl StorageBackend for MemoryBackend {
    fn get(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let data = self.data.read().unwrap();
        Ok(data.get(cf)
            .and_then(|cf_data| cf_data.get(key))
            .cloned())
    }
    
    fn put(&self, cf: &str, key: &[u8], value: &[u8]) -> Result<()> {
        let mut data = self.data.write().unwrap();
        data.entry(cf.to_string())
            .or_insert_with(HashMap::new)
            .insert(key.to_vec(), value.to_vec());
        Ok(())
    }
    
    fn delete(&self, cf: &str, key: &[u8]) -> Result<()> {
        let mut data = self.data.write().unwrap();
        if let Some(cf_data) = data.get_mut(cf) {
            cf_data.remove(key);
        }
        Ok(())
    }
    
    fn exists(&self, cf: &str, key: &[u8]) -> Result<bool> {
        Ok(self.get(cf, key)?.is_some())
    }
    
    fn scan_prefix(&self, cf: &str, prefix: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        let data = self.data.read().unwrap();
        Ok(data.get(cf)
            .map(|cf_data| {
                cf_data.iter()
                    .filter(|(k, _)| k.starts_with(prefix))
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect()
            })
            .unwrap_or_default())
    }
    
    fn flush(&self) -> Result<()> {
        // No-op for in-memory
        Ok(())
    }
    
    // Implement Transaction and Snapshot traits...
}
```

### Using Custom Backend

```rust
let backend = Arc::new(MemoryBackend::new());
let db = OpenDB::with_backend(backend, OpenDBOptions::default())?;
```

## Custom Cache Implementations

Implement the `Cache` trait for custom caching strategies:

```rust
pub trait Cache<K, V>: Send + Sync {
    fn get(&self, key: &K) -> Option<V>;
    fn put(&self, key: K, value: V);
    fn remove(&self, key: &K);
    fn clear(&self);
    fn len(&self) -> usize;
}
```

### Example: TTL Cache

```rust
use std::collections::HashMap;
use std::time::{Instant, Duration};
use parking_lot::RwLock;

pub struct TtlCache<K, V> {
    data: RwLock<HashMap<K, (V, Instant)>>,
    ttl: Duration,
}

impl<K: Eq + std::hash::Hash + Clone, V: Clone> Cache<K, V> for TtlCache<K, V> {
    fn get(&self, key: &K) -> Option<V> {
        let data = self.data.read();
        data.get(key).and_then(|(value, inserted)| {
            if inserted.elapsed() < self.ttl {
                Some(value.clone())
            } else {
                None // Expired
            }
        })
    }
    
    fn put(&self, key: K, value: V) {
        let mut data = self.data.write();
        data.insert(key, (value, Instant::now()));
    }
    
    // ... implement other methods
}
```

## Custom Vector Indexes

While OpenDB uses HNSW, you can wrap alternative indexes:

### Example: Flat Index

```rust
pub struct FlatVectorIndex {
    vectors: RwLock<Vec<(String, Vec<f32>)>>,
}

impl FlatVectorIndex {
    pub fn search(&self, query: &[f32], top_k: usize) -> Vec<SearchResult> {
        let vectors = self.vectors.read();
        let mut results: Vec<_> = vectors.iter()
            .map(|(id, vec)| {
                let distance = euclidean_distance(query, vec);
                SearchResult { id: id.clone(), distance }
            })
            .collect();
        
        results.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        results.truncate(top_k);
        results
    }
}

fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt()
}
```

## Custom Serialization

Replace `rkyv` with custom codec:

```rust
pub trait Codec<T> {
    fn encode(&self, value: &T) -> Result<Vec<u8>>;
    fn decode(&self, bytes: &[u8]) -> Result<T>;
}

pub struct JsonCodec;

impl<T: serde::Serialize + serde::de::DeserializeOwned> Codec<T> for JsonCodec {
    fn encode(&self, value: &T) -> Result<Vec<u8>> {
        serde_json::to_vec(value).map_err(|e| Error::Codec(e.to_string()))
    }
    
    fn decode(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Codec(e.to_string()))
    }
}
```

## Plugin System (Future)

Planned plugin architecture:

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn init(&mut self, db: &OpenDB) -> Result<()>;
    fn on_insert(&self, memory: &Memory) -> Result<()>;
    fn on_delete(&self, id: &str) -> Result<()>;
    fn on_link(&self, edge: &Edge) -> Result<()>;
}

// Example: Audit logger plugin
pub struct AuditPlugin {
    log_file: Mutex<File>,
}

impl Plugin for AuditPlugin {
    fn on_insert(&self, memory: &Memory) -> Result<()> {
        let mut file = self.log_file.lock().unwrap();
        writeln!(file, "INSERT: {}", memory.id)?;
        Ok(())
    }
}
```

## Custom Relation Types

Extend graph relations for domain-specific needs:

```rust
pub mod custom_relations {
    pub const IMPLEMENTS: &str = "implements";
    pub const EXTENDS: &str = "extends";
    pub const DEPENDS_ON: &str = "depends_on";
    pub const TESTED_BY: &str = "tested_by";
}

use custom_relations::*;

db.link("MyStruct", "MyTrait", IMPLEMENTS)?;
db.link("ChildStruct", "ParentStruct", EXTENDS)?;
```

## Embedding Adapters

Create adapters for different embedding models:

```rust
pub trait EmbeddingModel {
    fn dimension(&self) -> usize;
    fn encode(&self, text: &str) -> Result<Vec<f32>>;
}

pub struct SentenceTransformerAdapter {
    // Python bindings via PyO3
}

impl EmbeddingModel for SentenceTransformerAdapter {
    fn dimension(&self) -> usize {
        384 // all-MiniLM-L6-v2
    }
    
    fn encode(&self, text: &str) -> Result<Vec<f32>> {
        // Call Python model
        todo!()
    }
}
```

## Future Extension Points

Planned extensibility features:

1. **Query Language:** SQL-like interface for complex queries
2. **Triggers:** Execute callbacks on events
3. **Views:** Virtual collections with custom logic
4. **Migrations:** Schema evolution helpers
5. **Replication:** Multi-instance synchronization

## Contributing Extensions

If you build a useful extension, consider contributing:

1. **Fork** the repository
2. **Create** a new module in `src/extensions/`
3. **Document** usage and API
4. **Add tests** for functionality
5. **Submit** a pull request

## Best Practices

1. **Follow trait contracts:** Implement all required methods
2. **Handle errors:** Use `Result<T, Error>` consistently
3. **Thread safety:** Use `Send + Sync` for shared state
4. **Document:** Provide clear documentation and examples
5. **Test:** Write comprehensive tests for custom components

## Examples

See the `examples/` directory for:

- `custom_backend.rs`: Alternative storage backend
- `plugin_example.rs`: Sample plugin implementation
- `custom_index.rs`: Alternative vector index

## Next

- [Contributing Guide](../contributing.md)
- [Architecture Overview](../architecture/overview.md)
