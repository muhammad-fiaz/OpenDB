# OpenDB

![OpenDB Logo](https://img.shields.io/badge/OpenDB-Hybrid%20Database-blue)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green)

**OpenDB** is a high-performance hybrid embedded database written in pure Rust, combining multiple database paradigms into a single, cohesive system.

## Features

- **🔑 Key-Value Store**: Fast point lookups and range scans
- **📄 Structured Records**: Document/row storage with schema support
- **🔗 Graph Database**: Relationships and graph traversals
- **🔍 Vector Search**: Semantic search with HNSW-based approximate nearest neighbors
- **💾 In-Memory Cache**: LRU cache for hot data
- **✅ ACID Transactions**: Full transactional guarantees with WAL

## Why OpenDB?

OpenDB is designed for applications that need multiple database capabilities without the complexity of managing separate systems:

- **Agent Memory Systems**: Store and recall facts, relationships, and semantic information
- **Knowledge Graphs**: Build and traverse complex relationship networks
- **Semantic Search**: Find similar content using vector embeddings
- **High-Performance Applications**: LSM-tree backend for excellent write throughput

## Repository

- **GitHub**: [muhammad-fiaz/OpenDB](https://github.com/muhammad-fiaz/OpenDB)
- **Documentation**: [https://muhammad-fiaz.github.io/opendb](https://muhammad-fiaz.github.io/opendb)
- **Contact**: <contact@muhammadfiaz.com>

## Quick Example

```rust
use opendb::{OpenDB, Memory};

fn main() -> opendb::Result<()> {
    // Open database
    let db = OpenDB::open("./my_database")?;
    
    // Store a memory with embedding
    let memory = Memory::new(
        "memory_1",
        "Rust is awesome!",
        vec![0.1, 0.2, 0.3],
        0.9, // importance
    );
    db.insert_memory(&memory)?;
    
    // Create relationships
    db.link("memory_1", "related_to", "memory_2")?;
    
    // Vector search
    let similar = db.search_similar(&[0.1, 0.2, 0.3], 5)?;
    
    Ok(())
}
```

## Next Steps

- [Installation Guide](installation.md)
- [Quick Start Tutorial](quickstart.md)
- [Architecture Overview](architecture/overview.md)
