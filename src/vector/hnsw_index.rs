// HNSW index wrapper and utilities

/// HNSW search parameters
#[allow(dead_code)]
pub struct HnswParams {
    /// Maximum number of connections per layer (M)
    pub max_connections: usize,
    
    /// Size of the dynamic candidate list (ef_construction)
    pub ef_construction: usize,
    
    /// Search quality parameter (ef)
    pub ef_search: usize,
}

impl Default for HnswParams {
    fn default() -> Self {
        Self {
            max_connections: 16,
            ef_construction: 200,
            ef_search: 50,
        }
    }
}

#[allow(dead_code)]
impl HnswParams {
    /// Create params optimized for accuracy
    pub fn high_accuracy() -> Self {
        Self {
            max_connections: 32,
            ef_construction: 400,
            ef_search: 100,
        }
    }

    /// Create params optimized for speed
    pub fn high_speed() -> Self {
        Self {
            max_connections: 8,
            ef_construction: 100,
            ef_search: 25,
        }
    }
}
