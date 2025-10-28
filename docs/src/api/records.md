# Records API

The Records API manages structured `Memory` objects with metadata, timestamps, and embeddings.

## Memory Type

```rust
pub struct Memory {
    pub id: String,
    pub content: String,
    pub embedding: Vec<f32>,
    pub importance: f64,
    pub timestamp: i64,
    pub metadata: HashMap<String, String>,
}
```

## Creating Memories

### New Memory

```rust
use opendb::{OpenDB, Memory};

let memory = Memory::new(
    "mem_001".to_string(),
    "User asked about Rust ownership".to_string(),
);
```

### With Metadata

```rust
let memory = Memory::new("mem_002".to_string(), "Content".to_string())
    .with_metadata("category", "conversation")
    .with_metadata("user_id", "123");
```

### Custom Builder

```rust
use std::collections::HashMap;

let mut metadata = HashMap::new();
metadata.insert("priority".to_string(), "high".to_string());

let memory = Memory {
    id: "mem_003".to_string(),
    content: "Important note".to_string(),
    embedding: vec![0.1, 0.2, 0.3], // 3D for demo
    importance: 0.95,
    timestamp: chrono::Utc::now().timestamp(),
    metadata,
};
```

## CRUD Operations

### Insert

```rust
let db = OpenDB::open("./db")?;
let memory = Memory::new("mem_001".to_string(), "Hello world".to_string());
db.insert_memory(&memory)?;
```

**Signature:**

```rust
pub fn insert_memory(&self, memory: &Memory) -> Result<()>
```

**Behavior:**

- Serializes with `rkyv` (zero-copy)
- Writes to `records` column family
- Updates cache
- If embedding is non-empty, stores in vector index (requires rebuild for search)

### Get

```rust
let memory = db.get_memory("mem_001")?;
match memory {
    Some(mem) => println!("Content: {}", mem.content),
    None => println!("Not found"),
}
```

**Signature:**

```rust
pub fn get_memory(&self, id: &str) -> Result<Option<Memory>>
```

**Behavior:**

- Checks cache first
- Deserializes from storage on cache miss
- Returns `None` if not found

### Update

```rust
let mut memory = db.get_memory("mem_001")?.unwrap();
memory.content = "Updated content".to_string();
memory.importance = 0.9;
memory.touch(); // Update timestamp
db.insert_memory(&memory)?; // Upsert
```

**Note:** `insert_memory()` acts as upsert (update if exists, insert if not).

### Delete

```rust
db.delete_memory("mem_001")?;
```

**Signature:**

```rust
pub fn delete_memory(&self, id: &str) -> Result<()>
```

**Behavior:**

- Removes from storage
- Invalidates cache
- Does **not** remove from vector index (requires rebuild)
- Does **not** remove graph edges (handle separately)

## Listing Operations

### List All IDs

```rust
let ids = db.list_memory_ids()?;
for id in ids {
    println!("Memory ID: {}", id);
}
```

**Signature:**

```rust
pub fn list_memory_ids(&self) -> Result<Vec<String>>
```

### List All Memories

```rust
let memories = db.list_memories()?;
for memory in memories {
    println!("{}: {}", memory.id, memory.content);
}
```

**Signature:**

```rust
pub fn list_memories(&self) -> Result<Vec<Memory>>
```

**Warning:** Loads all memories into memory. For large datasets, use pagination (not yet implemented) or filter by prefix.

## Advanced Usage

### Importance Filtering

```rust
let memories = db.list_memories()?;
let important: Vec<_> = memories.into_iter()
    .filter(|m| m.importance > 0.8)
    .collect();
```

### Metadata Queries

```rust
let memories = db.list_memories()?;
let category_matches: Vec<_> = memories.into_iter()
    .filter(|m| {
        m.metadata.get("category")
            .map(|v| v == "conversation")
            .unwrap_or(false)
    })
    .collect();
```

### Time Range Queries

```rust
use chrono::{Utc, Duration};

let one_hour_ago = (Utc::now() - Duration::hours(1)).timestamp();
let recent: Vec<_> = db.list_memories()?.into_iter()
    .filter(|m| m.timestamp > one_hour_ago)
    .collect();
```

## Embeddings

### Setting Embeddings

Embeddings enable semantic search:

```rust
let embedding = generate_embedding("Hello world"); // Your embedding model
let memory = Memory {
    id: "mem_001".to_string(),
    content: "Hello world".to_string(),
    embedding, // Vec<f32>
    ..Default::default()
};
db.insert_memory(&memory)?;
```

### Dimension Requirements

All embeddings must have the same dimension (default 384):

```rust
use opendb::OpenDBOptions;

let mut options = OpenDBOptions::default();
options.vector_dimension = 768; // For larger models
let db = OpenDB::open_with_options("./db", options)?;
```

### Searching Embeddings

See [Vector API](vector.md) for semantic search.

## Touch Timestamp

Update access time without modifying content:

```rust
let mut memory = db.get_memory("mem_001")?.unwrap();
memory.touch(); // Sets timestamp to now
db.insert_memory(&memory)?;
```

## Default Values

```rust
impl Default for Memory {
    fn default() -> Self {
        Self {
            id: String::new(),
            content: String::new(),
            embedding: Vec::new(),
            importance: 0.5,
            timestamp: chrono::Utc::now().timestamp(),
            metadata: HashMap::new(),
        }
    }
}
```

## Performance Tips

1. **Batch Inserts:** Use transactions for multiple inserts:

```rust
let mut txn = db.begin_transaction()?;
for memory in memories {
    // Insert via transaction (lower-level API needed)
}
txn.commit()?;
```

2. **Cache Warm-Up:** Preload frequently accessed memories:

```rust
for id in important_ids {
    db.get_memory(id)?; // Populate cache
}
```

3. **Lazy Embedding Generation:** Only generate embeddings when needed for search:

```rust
let memory = Memory::new(id, content);
// Don't set embedding unless search is required
db.insert_memory(&memory)?;
```

## Error Handling

```rust
use opendb::Error;

match db.get_memory("mem_001") {
    Ok(Some(memory)) => { /* use memory */ },
    Ok(None) => { /* not found */ },
    Err(Error::Codec(_)) => { /* deserialization error */ },
    Err(Error::Storage(_)) => { /* storage error */ },
    Err(e) => { /* other error */ },
}
```

## Next

- [Graph API](graph.md)
- [Vector API](vector.md)
- [Transactions](transactions.md)
