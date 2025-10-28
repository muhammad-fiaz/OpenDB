// Records management for structured Memory data

use crate::cache::lru_cache::LruMemoryCache;
use crate::codec;
use crate::error::Result;
use crate::storage::{SharedStorage, column_families::ColumnFamilies};
use crate::types::Memory;
use std::sync::Arc;

/// Records manager for Memory CRUD operations
pub struct RecordsManager {
    storage: SharedStorage,
    cache: Arc<LruMemoryCache<String, Memory>>,
}

impl RecordsManager {
    /// Create a new records manager
    pub fn new(storage: SharedStorage, cache_capacity: usize) -> Self {
        Self {
            storage,
            cache: Arc::new(LruMemoryCache::new(cache_capacity)),
        }
    }

    /// Insert or update a memory record
    pub fn put(&self, memory: &Memory) -> Result<()> {
        let key = memory.id.as_bytes();
        let value = codec::encode_memory(memory)?;

        // Write to storage
        self.storage.put(ColumnFamilies::RECORDS, key, &value)?;

        // Update cache
        self.cache.insert(memory.id.clone(), memory.clone());

        Ok(())
    }

    /// Get a memory record by ID
    pub fn get(&self, id: &str) -> Result<Option<Memory>> {
        // Check cache first
        if let Some(memory) = self.cache.get_cloned(&id.to_string()) {
            return Ok(Some(memory));
        }

        // Cache miss - fetch from storage
        let key = id.as_bytes();
        if let Some(bytes) = self.storage.get(ColumnFamilies::RECORDS, key)? {
            let memory = codec::decode_memory(&bytes)?;
            self.cache.insert(id.to_string(), memory.clone());
            Ok(Some(memory))
        } else {
            Ok(None)
        }
    }

    /// Delete a memory record
    pub fn delete(&self, id: &str) -> Result<()> {
        let key = id.as_bytes();

        // Delete from storage
        self.storage.delete(ColumnFamilies::RECORDS, key)?;

        // Invalidate cache
        self.cache.invalidate(&id.to_string());

        Ok(())
    }

    /// Check if a memory exists
    #[allow(dead_code)]
    pub fn exists(&self, id: &str) -> Result<bool> {
        Ok(self.get(id)?.is_some())
    }

    /// List all memory IDs with a given prefix
    pub fn list_ids(&self, prefix: &str) -> Result<Vec<String>> {
        let prefix_bytes = prefix.as_bytes();
        let pairs = self
            .storage
            .scan_prefix(ColumnFamilies::RECORDS, prefix_bytes)?;

        let ids = pairs
            .into_iter()
            .filter_map(|(key, _)| String::from_utf8(key).ok())
            .collect();

        Ok(ids)
    }

    /// List all memories with a given prefix
    pub fn list(&self, prefix: &str) -> Result<Vec<Memory>> {
        let prefix_bytes = prefix.as_bytes();
        let pairs = self
            .storage
            .scan_prefix(ColumnFamilies::RECORDS, prefix_bytes)?;

        let mut memories = Vec::new();
        for (_, value) in pairs {
            let memory = codec::decode_memory(&value)?;
            memories.push(memory);
        }

        Ok(memories)
    }
}
