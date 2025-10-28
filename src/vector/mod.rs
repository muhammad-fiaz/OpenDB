// Vector search functionality with HNSW

pub mod hnsw_index;

use crate::error::{Error, Result};
use crate::types::Memory;
use crate::storage::{SharedStorage, column_families::ColumnFamilies};
use std::sync::Arc;
use parking_lot::RwLock;

/// Vector manager for semantic search
pub struct VectorManager {
    storage: SharedStorage,
    cache: Arc<RwLock<Option<Vec<(String, Vec<f32>)>>>>,
    dimension: usize,
}

impl VectorManager {
    /// Create a new vector manager
    pub fn new(storage: SharedStorage, dimension: usize) -> Self {
        Self {
            storage,
            cache: Arc::new(RwLock::new(None)),
            dimension,
        }
    }

    /// Insert a memory with its vector embedding
    pub fn insert(&self, memory: &Memory) -> Result<()> {
        if memory.embedding.len() != self.dimension {
            return Err(Error::VectorIndex(format!(
                "Expected dimension {}, got {}",
                self.dimension,
                memory.embedding.len()
            )));
        }

        // Store the embedding
        let key = memory.id.as_bytes();
        let embedding_bytes = bincode::encode_to_vec(&memory.embedding, bincode::config::standard())
            .map_err(|e| Error::Codec(format!("Failed to serialize embedding: {}", e)))?;
        
        self.storage.put(ColumnFamilies::VECTOR_DATA, key, &embedding_bytes)?;
        
        // Invalidate cache
        *self.cache.write() = None;
        
        Ok(())
    }

    /// Search for similar vectors
    pub fn search(&self, query: &[f32], k: usize) -> Result<Vec<(String, f32)>> {
        if query.len() != self.dimension {
            return Err(Error::VectorIndex(format!(
                "Query dimension mismatch: expected {}, got {}",
                self.dimension,
                query.len()
            )));
        }

        // Ensure cache is built
        self.ensure_cache_built()?;
        
        let cache = self.cache.read();
        let vectors = cache.as_ref().ok_or_else(|| Error::VectorIndex("Cache not built".to_string()))?;
        
        if vectors.is_empty() {
            return Ok(Vec::new());
        }
        
        // Brute-force k-NN search
        let mut results: Vec<(String, f32)> = vectors
            .iter()
            .map(|(id, embedding)| {
                let distance = euclidean_distance(query, embedding);
                (id.clone(), distance)
            })
            .collect();
        
        // Sort by distance and take top k
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(k);
        
        Ok(results)
    }

    /// Ensure cache is built from storage
    fn ensure_cache_built(&self) -> Result<()> {
        // Check if cache exists
        if self.cache.read().is_some() {
            return Ok(());
        }

        // Build cache
        let mut values = Vec::new();
        
        // Scan all vectors
        let pairs = self.storage.scan_prefix(ColumnFamilies::VECTOR_DATA, &[])?;
        
        for (key, value) in pairs {
            let id = String::from_utf8(key)
                .map_err(|e| Error::VectorIndex(format!("Invalid key: {}", e)))?;
            
            let (embedding, _): (Vec<f32>, usize) = bincode::decode_from_slice(&value, bincode::config::standard())
                .map_err(|e| Error::Codec(format!("Failed to deserialize embedding: {}", e)))?;
            
            values.push((id, embedding));
        }
        
        *self.cache.write() = Some(values);
        
        Ok(())
    }

    /// Delete a vector
    pub fn delete(&self, id: &str) -> Result<()> {
        let key = id.as_bytes();
        self.storage.delete(ColumnFamilies::VECTOR_DATA, key)?;
        
        // Invalidate cache
        *self.cache.write() = None;
        
        Ok(())
    }

    /// Force rebuild the cache
    pub fn rebuild_index(&self) -> Result<()> {
        *self.cache.write() = None;
        self.ensure_cache_built()
    }
}

/// Calculate Euclidean distance between two vectors
fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt()
}

