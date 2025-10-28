// Integration tests for OpenDB

use opendb::{Memory, OpenDB, OpenDBOptions, Result};
use tempfile::TempDir;

fn setup_test_db() -> Result<(OpenDB, TempDir)> {
    let temp_dir = TempDir::new().unwrap();
    // Use dimension 3 for tests to keep vectors small
    let options = OpenDBOptions::with_dimension(3);
    let db = OpenDB::open_with_options(temp_dir.path(), options)?;
    Ok((db, temp_dir))
}

#[test]
fn test_basic_kv_operations() -> Result<()> {
    let (db, _temp) = setup_test_db()?;

    // Put
    db.put(b"key1", b"value1")?;

    // Get
    let value = db.get(b"key1")?;
    assert_eq!(value, Some(b"value1".to_vec()));

    // Exists
    assert!(db.exists(b"key1")?);
    assert!(!db.exists(b"nonexistent")?);

    // Delete
    db.delete(b"key1")?;
    assert_eq!(db.get(b"key1")?, None);

    Ok(())
}

#[test]
fn test_memory_crud() -> Result<()> {
    let (db, _temp) = setup_test_db()?;

    // Create
    let memory = Memory::new("test_id", "test content", vec![1.0, 2.0, 3.0], 0.8);
    db.insert_memory(&memory)?;

    // Read
    let retrieved = db.get_memory("test_id")?;
    assert!(retrieved.is_some());
    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.id, "test_id");
    assert_eq!(retrieved.content, "test content");
    assert_eq!(retrieved.embedding, vec![1.0, 2.0, 3.0]);
    assert_eq!(retrieved.importance, 0.8);

    // Update
    let updated = Memory::new("test_id", "updated content", vec![4.0, 5.0, 6.0], 0.9);
    db.insert_memory(&updated)?;

    let retrieved = db.get_memory("test_id")?.unwrap();
    assert_eq!(retrieved.content, "updated content");
    assert_eq!(retrieved.importance, 0.9);

    // Delete
    db.delete_memory("test_id")?;
    assert!(db.get_memory("test_id")?.is_none());

    Ok(())
}

#[test]
fn test_graph_operations() -> Result<()> {
    let (db, _temp) = setup_test_db()?;

    // Create some memories
    let mem1 = Memory::new("mem1", "content1", vec![1.0; 3], 0.5);
    let mem2 = Memory::new("mem2", "content2", vec![2.0; 3], 0.5);
    let mem3 = Memory::new("mem3", "content3", vec![3.0; 3], 0.5);

    db.insert_memory(&mem1)?;
    db.insert_memory(&mem2)?;
    db.insert_memory(&mem3)?;

    // Link
    db.link("mem1", "related_to", "mem2")?;
    db.link("mem1", "references", "mem3")?;
    db.link("mem2", "related_to", "mem3")?;

    // Query relationships
    let related = db.get_related("mem1", "related_to")?;
    assert_eq!(related, vec!["mem2"]);

    let referenced = db.get_related("mem1", "references")?;
    assert_eq!(referenced, vec!["mem3"]);

    // Outgoing edges
    let outgoing = db.get_outgoing("mem1")?;
    assert_eq!(outgoing.len(), 2);

    // Incoming edges
    let incoming = db.get_incoming("mem3")?;
    assert_eq!(incoming.len(), 2);

    // Unlink
    db.unlink("mem1", "related_to", "mem2")?;
    let related = db.get_related("mem1", "related_to")?;
    assert!(related.is_empty());

    Ok(())
}

#[test]
fn test_vector_search() -> Result<()> {
    let (db, _temp) = setup_test_db()?;

    // Insert memories with different embeddings (3D to match test config)
    let mem1 = Memory::new("v1", "first", vec![1.0, 0.0, 0.0], 0.5);
    let mem2 = Memory::new("v2", "second", vec![0.9, 0.1, 0.0], 0.5);
    let mem3 = Memory::new("v3", "third", vec![0.0, 1.0, 0.0], 0.5);

    db.insert_memory(&mem1)?;
    db.insert_memory(&mem2)?;
    db.insert_memory(&mem3)?;

    // Search for similar to [1, 0, 0]
    let results = db.search_similar(&[1.0, 0.0, 0.0], 2)?;
    assert_eq!(results.len(), 2);

    // First result should be v1 (exact match)
    assert_eq!(results[0].id, "v1");
    assert!(results[0].distance < 0.01); // Very close to 0

    // Second should be v2 (close)
    assert_eq!(results[1].id, "v2");

    Ok(())
}

#[test]
fn test_list_operations() -> Result<()> {
    let (db, _temp) = setup_test_db()?;

    // Insert with prefixes
    db.insert_memory(&Memory::new("user_001", "content", vec![1.0; 3], 0.5))?;
    db.insert_memory(&Memory::new("user_002", "content", vec![1.0; 3], 0.5))?;
    db.insert_memory(&Memory::new("system_001", "content", vec![1.0; 3], 0.5))?;

    // List with prefix
    let user_ids = db.list_memory_ids("user")?;
    assert_eq!(user_ids.len(), 2);
    assert!(user_ids.contains(&"user_001".to_string()));
    assert!(user_ids.contains(&"user_002".to_string()));

    let user_memories = db.list_memories("user")?;
    assert_eq!(user_memories.len(), 2);

    Ok(())
}

#[test]
fn test_cache_coherency() -> Result<()> {
    let (db, _temp) = setup_test_db()?;

    // Insert
    let mem = Memory::new("cache_test", "original", vec![1.0; 3], 0.5);
    db.insert_memory(&mem)?;

    // Read (should be cached)
    let retrieved1 = db.get_memory("cache_test")?.unwrap();
    assert_eq!(retrieved1.content, "original");

    // Update
    let updated = Memory::new("cache_test", "updated", vec![2.0; 3], 0.6);
    db.insert_memory(&updated)?;

    // Read again (should get updated value)
    let retrieved2 = db.get_memory("cache_test")?.unwrap();
    assert_eq!(retrieved2.content, "updated");
    assert_eq!(retrieved2.importance, 0.6);

    // Delete
    db.delete_memory("cache_test")?;

    // Should not be found
    assert!(db.get_memory("cache_test")?.is_none());

    Ok(())
}

#[test]
fn test_metadata() -> Result<()> {
    let (db, _temp) = setup_test_db()?;

    let mem = Memory::new("meta_test", "content", vec![1.0; 3], 0.5)
        .with_metadata("key1", "value1")
        .with_metadata("key2", "value2");

    db.insert_memory(&mem)?;

    let retrieved = db.get_memory("meta_test")?.unwrap();
    assert_eq!(retrieved.metadata.get("key1"), Some(&"value1".to_string()));
    assert_eq!(retrieved.metadata.get("key2"), Some(&"value2".to_string()));

    Ok(())
}
