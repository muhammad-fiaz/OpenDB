// Column family definitions for RocksDB
//
// Column families provide namespace isolation for different data types.

/// Column family names used in OpenDB
pub struct ColumnFamilies;

impl ColumnFamilies {
    /// Default column family (key-value store)
    pub const DEFAULT: &'static str = "default";

    /// Memory records
    pub const RECORDS: &'static str = "records";

    /// Forward graph index (from_id -> [(relation, to_id)])
    pub const GRAPH_FORWARD: &'static str = "graph_forward";

    /// Backward graph index (to_id -> [(relation, from_id)])
    pub const GRAPH_BACKWARD: &'static str = "graph_backward";

    /// Vector index metadata
    pub const VECTOR_INDEX: &'static str = "vector_index";

    /// Vector data (id -> embedding)
    pub const VECTOR_DATA: &'static str = "vector_data";

    /// Database metadata
    pub const METADATA: &'static str = "metadata";

    /// Get all column family names
    pub fn all() -> Vec<&'static str> {
        vec![
            Self::DEFAULT,
            Self::RECORDS,
            Self::GRAPH_FORWARD,
            Self::GRAPH_BACKWARD,
            Self::VECTOR_INDEX,
            Self::VECTOR_DATA,
            Self::METADATA,
        ]
    }
}
