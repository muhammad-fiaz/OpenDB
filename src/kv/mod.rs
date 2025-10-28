// Key-Value store API

use crate::error::Result;
use crate::storage::{SharedStorage, column_families::ColumnFamilies};
use crate::cache::lru_cache::LruMemoryCache;
use std::sync::Arc;

/// Key-Value store
pub struct KvStore {
    storage: SharedStorage,
    cache: Arc<LruMemoryCache<Vec<u8>, Vec<u8>>>,
}

impl KvStore {
    /// Create a new KV store
    pub fn new(storage: SharedStorage, cache_capacity: usize) -> Self {
        Self {
            storage,
            cache: Arc::new(LruMemoryCache::new(cache_capacity)),
        }
    }

    /// Get a value by key
    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        // Check cache first
        if let Some(value) = self.cache.get_cloned(&key.to_vec()) {
            return Ok(Some(value));
        }

        // Cache miss - fetch from storage
        if let Some(value) = self.storage.get(ColumnFamilies::DEFAULT, key)? {
            self.cache.insert(key.to_vec(), value.clone());
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    /// Put a key-value pair
    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        // Write-through: update storage first
        self.storage.put(ColumnFamilies::DEFAULT, key, value)?;
        
        // Then update cache
        self.cache.insert(key.to_vec(), value.to_vec());
        
        Ok(())
    }

    /// Delete a key
    pub fn delete(&self, key: &[u8]) -> Result<()> {
        // Delete from storage
        self.storage.delete(ColumnFamilies::DEFAULT, key)?;
        
        // Invalidate cache
        self.cache.invalidate(&key.to_vec());
        
        Ok(())
    }

    /// Check if a key exists
    pub fn exists(&self, key: &[u8]) -> Result<bool> {
        Ok(self.get(key)?.is_some())
    }

    /// Scan keys with a prefix
    pub fn scan_prefix(&self, prefix: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        self.storage.scan_prefix(ColumnFamilies::DEFAULT, prefix)
    }
}
