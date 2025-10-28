// Graph database functionality

pub mod relation;

use crate::error::Result;
use crate::types::Edge;
use crate::storage::{SharedStorage, column_families::ColumnFamilies};
use crate::codec;

/// Graph manager for relationship operations
pub struct GraphManager {
    storage: SharedStorage,
}

impl GraphManager {
    /// Create a new graph manager
    pub fn new(storage: SharedStorage) -> Self {
        Self { storage }
    }

    /// Create a link between two entities
    ///
    /// # Arguments
    ///
    /// * `from` - Source entity ID
    /// * `relation` - Relationship type
    /// * `to` - Target entity ID
    pub fn link(&self, from: &str, relation: &str, to: &str) -> Result<()> {
        let edge = Edge::new(from, relation, to);
        
        // Store in forward index (from -> to)
        self.add_to_adjacency_list(
            ColumnFamilies::GRAPH_FORWARD,
            &edge.from,
            &edge,
        )?;
        
        // Store in backward index (to -> from)
        self.add_to_adjacency_list(
            ColumnFamilies::GRAPH_BACKWARD,
            &edge.to,
            &edge,
        )?;
        
        Ok(())
    }

    /// Remove a link between two entities
    pub fn unlink(&self, from: &str, relation: &str, to: &str) -> Result<()> {
        // Remove from forward index
        self.remove_from_adjacency_list(
            ColumnFamilies::GRAPH_FORWARD,
            from,
            relation,
            to,
        )?;
        
        // Remove from backward index
        self.remove_from_adjacency_list(
            ColumnFamilies::GRAPH_BACKWARD,
            to,
            relation,
            from,
        )?;
        
        Ok(())
    }

    /// Get all outgoing edges from an entity
    pub fn get_outgoing(&self, from: &str, relation: Option<&str>) -> Result<Vec<Edge>> {
        self.get_edges(ColumnFamilies::GRAPH_FORWARD, from, relation)
    }

    /// Get all incoming edges to an entity
    pub fn get_incoming(&self, to: &str, relation: Option<&str>) -> Result<Vec<Edge>> {
        self.get_edges(ColumnFamilies::GRAPH_BACKWARD, to, relation)
    }

    /// Get related entity IDs
    pub fn get_related(&self, id: &str, relation: &str) -> Result<Vec<String>> {
        let edges = self.get_outgoing(id, Some(relation))?;
        Ok(edges.into_iter().map(|e| e.to).collect())
    }

    /// Helper: Add edge to adjacency list
    fn add_to_adjacency_list(&self, cf: &str, key: &str, edge: &Edge) -> Result<()> {
        let key_bytes = key.as_bytes();
        
        // Get existing edges
        let mut edges = if let Some(bytes) = self.storage.get(cf, key_bytes)? {
            codec::decode_edges(&bytes)?
        } else {
            Vec::new()
        };
        
        // Add new edge (avoid duplicates)
        if !edges.iter().any(|e| e.from == edge.from && e.to == edge.to && e.relation == edge.relation) {
            edges.push(edge.clone());
        }
        
        // Store back
        let encoded = codec::encode_edges(&edges)?;
        self.storage.put(cf, key_bytes, &encoded)?;
        
        Ok(())
    }

    /// Helper: Remove edge from adjacency list
    fn remove_from_adjacency_list(&self, cf: &str, key: &str, relation: &str, target: &str) -> Result<()> {
        let key_bytes = key.as_bytes();
        
        // Get existing edges
        let mut edges = if let Some(bytes) = self.storage.get(cf, key_bytes)? {
            codec::decode_edges(&bytes)?
        } else {
            return Ok(()); // Nothing to remove
        };
        
        // Remove matching edges
        edges.retain(|e| !(e.relation == relation && (e.from == target || e.to == target)));
        
        // Store back
        if edges.is_empty() {
            self.storage.delete(cf, key_bytes)?;
        } else {
            let encoded = codec::encode_edges(&edges)?;
            self.storage.put(cf, key_bytes, &encoded)?;
        }
        
        Ok(())
    }

    /// Helper: Get edges for an entity
    fn get_edges(&self, cf: &str, key: &str, relation: Option<&str>) -> Result<Vec<Edge>> {
        let key_bytes = key.as_bytes();
        
        let edges = if let Some(bytes) = self.storage.get(cf, key_bytes)? {
            codec::decode_edges(&bytes)?
        } else {
            Vec::new()
        };
        
        // Filter by relation if specified
        if let Some(rel) = relation {
            Ok(edges.into_iter().filter(|e| e.relation == rel).collect())
        } else {
            Ok(edges)
        }
    }
}
