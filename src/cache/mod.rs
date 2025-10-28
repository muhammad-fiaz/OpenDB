// In-memory cache layer
//
// This module provides LRU caching for hot data with write-through semantics.

pub mod lru_cache;

/// Cache trait for different caching strategies
#[allow(dead_code)]
pub trait Cache<K, V>: Send + Sync {
    /// Get a value from cache
    fn get(&mut self, key: &K) -> Option<&V>;

    /// Put a value into cache
    fn put(&mut self, key: K, value: V);

    /// Remove a value from cache
    fn remove(&mut self, key: &K) -> Option<V>;

    /// Clear the entire cache
    fn clear(&mut self);

    /// Get cache size
    fn len(&self) -> usize;

    /// Check if cache is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
