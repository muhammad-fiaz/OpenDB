// Serialization codec for OpenDB
//
// This module handles encoding and decoding of data structures
// using rkyv for zero-copy deserialization.

use crate::error::{Error, Result};
use crate::types::{Edge, Memory};
use rkyv::{AlignedVec, Deserialize};

/// Schema version for backwards compatibility
const SCHEMA_VERSION: u8 = 1;

/// Encode a Memory record
pub fn encode_memory(memory: &Memory) -> Result<Vec<u8>> {
    let bytes = rkyv::to_bytes::<_, 256>(memory)
        .map_err(|e| Error::Codec(format!("Failed to serialize Memory: {}", e)))?;

    // Prepend schema version
    let mut result = vec![SCHEMA_VERSION];
    result.extend_from_slice(&bytes);
    Ok(result)
}

/// Decode a Memory record
pub fn decode_memory(bytes: &[u8]) -> Result<Memory> {
    if bytes.is_empty() {
        return Err(Error::Codec("Empty byte array".to_string()));
    }

    // Check schema version
    let version = bytes[0];
    if version != SCHEMA_VERSION {
        return Err(Error::Codec(format!(
            "Unsupported schema version: {}",
            version
        )));
    }

    let data = &bytes[1..];

    // Copy to aligned buffer for rkyv
    let mut aligned = AlignedVec::new();
    aligned.extend_from_slice(data);

    let archived = rkyv::check_archived_root::<Memory>(&aligned)
        .map_err(|e| Error::Codec(format!("Failed to validate archived Memory: {}", e)))?;

    let memory: Memory = archived
        .deserialize(&mut rkyv::Infallible)
        .map_err(|e| Error::Codec(format!("Failed to deserialize Memory: {}", e)))?;

    Ok(memory)
}

/// Encode an Edge
#[allow(dead_code)]
pub fn encode_edge(edge: &Edge) -> Result<Vec<u8>> {
    let bytes = rkyv::to_bytes::<_, 256>(edge)
        .map_err(|e| Error::Codec(format!("Failed to serialize Edge: {}", e)))?;

    let mut result = vec![SCHEMA_VERSION];
    result.extend_from_slice(&bytes);
    Ok(result)
}

/// Decode an Edge
#[allow(dead_code)]
pub fn decode_edge(bytes: &[u8]) -> Result<Edge> {
    if bytes.is_empty() {
        return Err(Error::Codec("Empty byte array".to_string()));
    }

    let version = bytes[0];
    if version != SCHEMA_VERSION {
        return Err(Error::Codec(format!(
            "Unsupported schema version: {}",
            version
        )));
    }

    let data = &bytes[1..];

    // Copy to aligned buffer for rkyv
    let mut aligned = AlignedVec::new();
    aligned.extend_from_slice(data);

    let archived = rkyv::check_archived_root::<Edge>(&aligned)
        .map_err(|e| Error::Codec(format!("Failed to validate archived Edge: {}", e)))?;

    let edge: Edge = archived
        .deserialize(&mut rkyv::Infallible)
        .map_err(|e| Error::Codec(format!("Failed to deserialize Edge: {}", e)))?;

    Ok(edge)
}

/// Encode a list of edges
pub fn encode_edges(edges: &[Edge]) -> Result<Vec<u8>> {
    let edges_vec: Vec<Edge> = edges.to_vec();
    let bytes = rkyv::to_bytes::<_, 256>(&edges_vec)
        .map_err(|e| Error::Codec(format!("Failed to serialize edges: {}", e)))?;

    let mut result = vec![SCHEMA_VERSION];
    result.extend_from_slice(&bytes);
    Ok(result)
}

/// Decode a list of edges
pub fn decode_edges(bytes: &[u8]) -> Result<Vec<Edge>> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }

    let version = bytes[0];
    if version != SCHEMA_VERSION {
        return Err(Error::Codec(format!(
            "Unsupported schema version: {}",
            version
        )));
    }

    let data = &bytes[1..];

    // Copy to aligned buffer for rkyv
    let mut aligned = AlignedVec::new();
    aligned.extend_from_slice(data);

    let archived = rkyv::check_archived_root::<Vec<Edge>>(&aligned)
        .map_err(|e| Error::Codec(format!("Failed to validate archived edges: {}", e)))?;

    let edges: Vec<Edge> = archived
        .deserialize(&mut rkyv::Infallible)
        .map_err(|e| Error::Codec(format!("Failed to deserialize edges: {}", e)))?;

    Ok(edges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_encode_decode() {
        let memory = Memory::new("test_id", "test content", vec![1.0, 2.0, 3.0], 0.5);

        let encoded = encode_memory(&memory).unwrap();
        let decoded = decode_memory(&encoded).unwrap();

        assert_eq!(memory.id, decoded.id);
        assert_eq!(memory.content, decoded.content);
        assert_eq!(memory.embedding, decoded.embedding);
        assert_eq!(memory.importance, decoded.importance);
    }

    #[test]
    fn test_edge_encode_decode() {
        let edge = Edge::new("from_1", "related", "to_1");

        let encoded = encode_edge(&edge).unwrap();
        let decoded = decode_edge(&encoded).unwrap();

        assert_eq!(edge.from, decoded.from);
        assert_eq!(edge.relation, decoded.relation);
        assert_eq!(edge.to, decoded.to);
    }
}
