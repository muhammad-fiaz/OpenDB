// Storage abstraction layer
//
// This module defines the storage traits that allow pluggable backends.

pub mod column_families;
pub mod rocksdb_backend;

use crate::error::Result;
use std::sync::Arc;

/// Storage backend trait
///
/// This trait abstracts the underlying storage engine, allowing
/// different implementations (RocksDB, redb, custom LSM, etc.)
pub trait StorageBackend: Send + Sync {
    /// Get a value by key from a column family
    fn get(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>>;

    /// Put a key-value pair into a column family
    fn put(&self, cf: &str, key: &[u8], value: &[u8]) -> Result<()>;

    /// Delete a key from a column family
    fn delete(&self, cf: &str, key: &[u8]) -> Result<()>;

    /// Check if a key exists in a column family
    #[allow(dead_code)]
    fn exists(&self, cf: &str, key: &[u8]) -> Result<bool> {
        Ok(self.get(cf, key)?.is_some())
    }

    /// Iterate over keys in a column family with a prefix
    fn scan_prefix(&self, cf: &str, prefix: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>>;

    /// Begin a transaction
    fn begin_transaction(&self) -> Result<Box<dyn Transaction>>;

    /// Flush writes to disk
    fn flush(&self) -> Result<()>;

    /// Create a snapshot for consistent reads
    #[allow(dead_code)]
    fn snapshot(&self) -> Result<Box<dyn Snapshot>>;
}

/// Transaction trait for ACID operations
pub trait Transaction: Send {
    /// Get a value within this transaction
    fn get(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>>;

    /// Put a key-value pair within this transaction
    fn put(&mut self, cf: &str, key: &[u8], value: &[u8]) -> Result<()>;

    /// Delete a key within this transaction
    fn delete(&mut self, cf: &str, key: &[u8]) -> Result<()>;

    /// Commit the transaction
    fn commit(self: Box<Self>) -> Result<()>;

    /// Rollback the transaction
    fn rollback(self: Box<Self>) -> Result<()>;
}

/// Snapshot trait for consistent point-in-time reads
#[allow(dead_code)]
pub trait Snapshot: Send + Sync {
    /// Get a value from this snapshot
    fn get(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>>;
}

/// Type alias for a thread-safe storage backend
pub type SharedStorage = Arc<dyn StorageBackend>;
