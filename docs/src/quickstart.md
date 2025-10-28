# Quick Start

This guide will walk you through the basic usage of OpenDB.

## Opening a Database

```rust
use opendb::{OpenDB, Result};

fn main() -> Result<()> {
    // Open or create a database
    let db = OpenDB::open("./my_database")?;
    Ok(())
}
```

## Working with Key-Value Data

```rust
// Store a value
db.put(b"my_key", b"my_value")?;

// Retrieve a value
if let Some(value) = db.get(b"my_key")? {
    println!("Value: {:?}", value);
}

// Delete a value
db.delete(b"my_key")?;

// Check existence
if db.exists(b"my_key")? {
    println!("Key exists!");
}
```

## Working with Memory Records

Memory records are structured data with embeddings for semantic search.

```rust
use opendb::Memory;

// Create a memory
let memory = Memory::new(
    "memory_001",
    "The user prefers dark mode",
    vec![0.1, 0.2, 0.3, 0.4], // embedding vector
    0.9, // importance (0.0 to 1.0)
)
.with_metadata("category", "preference")
.with_metadata("source", "user_settings");

// Insert the memory
db.insert_memory(&memory)?;

// Retrieve it
if let Some(mem) = db.get_memory("memory_001")? {
    println!("Content: {}", mem.content);
    println!("Importance: {}", mem.importance);
}

// List all memories with a prefix
let all = db.list_memories("memory")?;
println!("Found {} memories", all.len());
```

## Creating Relationships

```rust
// Create relationships between memories
db.link("memory_001", "related_to", "memory_002")?;
db.link("memory_001", "caused_by", "memory_003")?;

// Query relationships
let related = db.get_related("memory_001", "related_to")?;
for id in related {
    println!("Related memory: {}", id);
}

// Get all outgoing edges
let edges = db.get_outgoing("memory_001")?;
for edge in edges {
    println!("{} --[{}]--> {}", edge.from, edge.relation, edge.to);
}
```

## Vector Search

```rust
// Search for similar memories
let query_embedding = vec![0.1, 0.2, 0.3, 0.4];
let results = db.search_similar(&query_embedding, 5)?; // top 5

for result in results {
    println!("Memory: {} (distance: {:.4})", 
             result.memory.content, 
             result.distance);
}
```

## Using Transactions

```rust
// Begin a transaction
let mut txn = db.begin_transaction()?;

// Perform operations
txn.put("records", b"key1", b"value1")?;
txn.put("records", b"key2", b"value2")?;

// Commit the transaction
txn.commit()?;

// Or rollback if needed
// txn.rollback()?;
```

## Flushing to Disk

```rust
// Ensure all writes are persisted
db.flush()?;
```

## Complete Example

See the [quickstart example](https://github.com/muhammad-fiaz/OpenDB/blob/main/examples/quickstart.rs) for a complete, runnable example.

## Next Steps

- [API Reference](api/kv.md)
- [Architecture Overview](architecture/overview.md)
- [Performance Tuning](advanced/performance.md)
