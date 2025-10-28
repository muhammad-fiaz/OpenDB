// OpenDB
//
// A high-performance hybrid embedded database written in Rust.
//
// OpenDB combines multiple database paradigms into a single, cohesive system:
// - **Key-Value Store**: Fast point lookups and scans
// - **Structured Records**: Document/row storage with schema
// - **Graph Database**: Relationships and traversals
// - **Vector Database**: Semantic search with approximate nearest neighbors
// - **In-Memory Cache**: LRU cache for hot data
// - **ACID Transactions**: Full transactional guarantees
//
// Quick Start
//
// ```rust,no_run
// use opendb::{OpenDB, Memory, Result};
//
// # fn main() -> Result<()> {
// // Open or create a database
// let db = OpenDB::open("./my_database")?;
//
// // Create a memory record
// let memory = Memory::new(
//     "memory_1",
//     "Rust is a systems programming language",
//     vec![0.1, 0.2, 0.3], // embedding vector
//     0.8, // importance
// );
//
// // Insert the record
// db.insert_memory(&memory)?;
//
// // Retrieve it
// let retrieved = db.get_memory("memory_1")?;
//
// // Create relationships
// db.link("memory_1", "related_to", "memory_2")?;
//
// // Vector search
// let similar = db.search_similar(&[0.1, 0.2, 0.3], 5)?;
// # Ok(())
// # }
// ```
//
// Repository
//
// - GitHub: <https://github.com/muhammad-fiaz/opendb>
// - Documentation: <https://muhammad-fiaz.github.io/opendb>
// - Contact: <contact@muhammadfiaz.com>

// Re-export main types
pub use database::{OpenDB, OpenDBOptions};
pub use error::{Error, Result};
pub use types::{
    DocumentChunk, FileType, Memory, MemoryMetadata, MultimodalDocument, ProcessingStatus,
};

// Core modules
pub mod database;
pub mod error;
pub mod types;

// Internal modules
pub(crate) mod cache;
pub(crate) mod codec;
pub(crate) mod graph;
pub(crate) mod kv;
pub(crate) mod records;
pub(crate) mod storage;
pub(crate) mod transaction;
pub(crate) mod vector;
