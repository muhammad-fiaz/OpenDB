// Main database module

use crate::error::Result;
use crate::graph::GraphManager;
use crate::kv::KvStore;
use crate::records::RecordsManager;
use crate::storage::{SharedStorage, rocksdb_backend::RocksDBBackend};
use crate::transaction::{Transaction, manager::TransactionManager};
use crate::types::{Memory, SearchResult};
use crate::vector::VectorManager;
use std::path::Path;
use std::sync::Arc;

/// OpenDB - High-performance hybrid embedded database
///
/// This is the main entry point for interacting with OpenDB.
///
/// # Example
///
/// ```no_run
/// use opendb::{OpenDB, Memory};
///
/// # fn main() -> opendb::Result<()> {
/// let db = OpenDB::open("./my_db")?;
///
/// let memory = Memory::new("id1", "content", vec![1.0, 2.0, 3.0], 0.8);
/// db.insert_memory(&memory)?;
///
/// let retrieved = db.get_memory("id1")?;
/// # Ok(())
/// # }
/// ```
pub struct OpenDB {
    storage: SharedStorage,
    kv: KvStore,
    records: RecordsManager,
    graph: GraphManager,
    vector: VectorManager,
    txn_manager: TransactionManager,
}

impl OpenDB {
    /// Open or create a database at the given path
    ///
    /// # Arguments
    ///
    /// * `path` - Database directory path
    ///
    /// # Returns
    ///
    /// A new OpenDB instance
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::open_with_options(path, OpenDBOptions::default())
    }

    /// Open with custom options
    pub fn open_with_options<P: AsRef<Path>>(path: P, options: OpenDBOptions) -> Result<Self> {
        let backend = RocksDBBackend::open(path)?;
        let storage: SharedStorage = Arc::new(backend);

        let kv = KvStore::new(Arc::clone(&storage), options.kv_cache_size);
        let records = RecordsManager::new(Arc::clone(&storage), options.record_cache_size);
        let graph = GraphManager::new(Arc::clone(&storage));
        let vector = VectorManager::new(Arc::clone(&storage), options.vector_dimension);
        let txn_manager = TransactionManager::new(Arc::clone(&storage));

        Ok(Self {
            storage,
            kv,
            records,
            graph,
            vector,
            txn_manager,
        })
    }

    // ===== Key-Value Operations =====

    /// Get a value by key
    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        self.kv.get(key)
    }

    /// Put a key-value pair
    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.kv.put(key, value)
    }

    /// Delete a key
    pub fn delete(&self, key: &[u8]) -> Result<()> {
        self.kv.delete(key)
    }

    /// Check if a key exists
    pub fn exists(&self, key: &[u8]) -> Result<bool> {
        self.kv.exists(key)
    }

    /// Scan keys with a prefix
    pub fn scan_prefix(&self, prefix: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        self.kv.scan_prefix(prefix)
    }

    // ===== Memory Record Operations =====

    /// Insert or update a memory record
    pub fn insert_memory(&self, memory: &Memory) -> Result<()> {
        // Store the record
        self.records.put(memory)?;

        // Index the vector
        self.vector.insert(memory)?;

        Ok(())
    }

    /// Get a memory record by ID
    pub fn get_memory(&self, id: &str) -> Result<Option<Memory>> {
        self.records.get(id)
    }

    /// Delete a memory record
    pub fn delete_memory(&self, id: &str) -> Result<()> {
        self.records.delete(id)?;
        self.vector.delete(id)?;
        Ok(())
    }

    /// List all memory IDs with a prefix
    pub fn list_memory_ids(&self, prefix: &str) -> Result<Vec<String>> {
        self.records.list_ids(prefix)
    }

    /// List all memories with a prefix
    pub fn list_memories(&self, prefix: &str) -> Result<Vec<Memory>> {
        self.records.list(prefix)
    }

    // ===== Graph Operations =====

    /// Create a link between two entities
    ///
    /// # Arguments
    ///
    /// * `from` - Source entity ID
    /// * `relation` - Relationship type
    /// * `to` - Target entity ID
    pub fn link(&self, from: &str, relation: &str, to: &str) -> Result<()> {
        self.graph.link(from, relation, to)
    }

    /// Remove a link
    pub fn unlink(&self, from: &str, relation: &str, to: &str) -> Result<()> {
        self.graph.unlink(from, relation, to)
    }

    /// Get related entity IDs
    pub fn get_related(&self, id: &str, relation: &str) -> Result<Vec<String>> {
        self.graph.get_related(id, relation)
    }

    /// Get all outgoing edges from an entity
    pub fn get_outgoing(&self, from: &str) -> Result<Vec<crate::types::Edge>> {
        self.graph.get_outgoing(from, None)
    }

    /// Get all incoming edges to an entity
    pub fn get_incoming(&self, to: &str) -> Result<Vec<crate::types::Edge>> {
        self.graph.get_incoming(to, None)
    }

    // ===== Vector Search Operations =====

    /// Search for similar memories by vector
    ///
    /// # Arguments
    ///
    /// * `query` - Query embedding vector
    /// * `k` - Number of results to return
    ///
    /// # Returns
    ///
    /// List of search results with distances
    pub fn search_similar(&self, query: &[f32], k: usize) -> Result<Vec<SearchResult>> {
        let results = self.vector.search(query, k)?;

        let mut search_results = Vec::new();
        for (id, distance) in results {
            if let Some(memory) = self.get_memory(&id)? {
                search_results.push(SearchResult {
                    id: id.clone(),
                    distance,
                    memory,
                });
            }
        }

        Ok(search_results)
    }

    /// Rebuild the vector index
    pub fn rebuild_vector_index(&self) -> Result<()> {
        self.vector.rebuild_index()
    }

    // ===== Transaction Operations =====

    /// Begin a new transaction
    pub fn begin_transaction(&self) -> Result<Transaction> {
        self.txn_manager.begin()
    }

    /// Flush all pending writes to disk
    pub fn flush(&self) -> Result<()> {
        self.storage.flush()
    }
}

/// Configuration options for OpenDB
#[derive(Debug, Clone)]
pub struct OpenDBOptions {
    /// KV cache size (number of entries)
    pub kv_cache_size: usize,

    /// Record cache size (number of entries)
    pub record_cache_size: usize,

    /// Vector dimension
    pub vector_dimension: usize,

    /// Database storage path (optional - will use path from open() if not set)
    pub storage_path: Option<String>,
}

impl Default for OpenDBOptions {
    fn default() -> Self {
        Self {
            kv_cache_size: 1000,
            record_cache_size: 500,
            vector_dimension: 384, // Common dimension for sentence transformers
            storage_path: None,
        }
    }
}

impl OpenDBOptions {
    /// Create new options with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create options with a specific vector dimension
    pub fn with_dimension(dimension: usize) -> Self {
        Self {
            vector_dimension: dimension,
            ..Default::default()
        }
    }

    /// Set vector dimension (chainable)
    pub fn dimension(mut self, dimension: usize) -> Self {
        self.vector_dimension = dimension;
        self
    }

    /// Set custom storage path (chainable)
    pub fn with_storage_path<S: Into<String>>(mut self, path: S) -> Self {
        self.storage_path = Some(path.into());
        self
    }

    /// Set KV cache size (chainable)
    pub fn with_kv_cache_size(mut self, size: usize) -> Self {
        self.kv_cache_size = size;
        self
    }

    /// Set record cache size (chainable)
    pub fn with_record_cache_size(mut self, size: usize) -> Self {
        self.record_cache_size = size;
        self
    }
}
