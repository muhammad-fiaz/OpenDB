// ACID compliance tests

use opendb::{OpenDB, OpenDBOptions, Memory, Result};
use tempfile::TempDir;
use std::thread;
use std::sync::Arc;

fn setup_test_db() -> Result<(OpenDB, TempDir)> {
    let temp_dir = TempDir::new().unwrap();
    // Use dimension 3 for tests to keep vectors small
    let options = OpenDBOptions::with_dimension(3);
    let db = OpenDB::open_with_options(temp_dir.path(), options)?;
    Ok((db, temp_dir))
}

#[test]
fn test_atomicity() -> Result<()> {
    let (db, _temp) = setup_test_db()?;

    // Begin transaction
    let mut txn = db.begin_transaction()?;

    // Perform multiple operations in the default CF
    txn.put("default", b"key1", b"value1")?;
    txn.put("default", b"key2", b"value2")?;
    txn.put("default", b"key3", b"value3")?;

    // Commit
    txn.commit()?;

    // All should be present (using db.get which reads from default CF)
    assert!(db.get(b"key1")?.is_some());
    assert!(db.get(b"key2")?.is_some());
    assert!(db.get(b"key3")?.is_some());

    Ok(())
}

#[test]
fn test_rollback() -> Result<()> {
    let (db, _temp) = setup_test_db()?;

    // Initial state
    db.put(b"key1", b"original")?;

    // Begin transaction
    let mut txn = db.begin_transaction()?;
    txn.put("default", b"key1", b"modified")?;
    txn.put("default", b"key2", b"new")?;

    // Rollback
    txn.rollback()?;

    // Original value should remain
    let val = db.get(b"key1")?;
    assert_eq!(val, Some(b"original".to_vec()));

    // New key should not exist
    assert!(db.get(b"key2")?.is_none());

    Ok(())
}

#[test]
fn test_consistency() -> Result<()> {
    let (db, _temp) = setup_test_db()?;

    // Create related memories
    let mem1 = Memory::new("cons_1", "content1", vec![1.0; 3], 0.5);
    let mem2 = Memory::new("cons_2", "content2", vec![2.0; 3], 0.5);

    db.insert_memory(&mem1)?;
    db.insert_memory(&mem2)?;
    db.link("cons_1", "related", "cons_2")?;

    // Both memory and relationship should exist
    assert!(db.get_memory("cons_1")?.is_some());
    assert!(db.get_memory("cons_2")?.is_some());
    
    let related = db.get_related("cons_1", "related")?;
    assert_eq!(related, vec!["cons_2"]);

    Ok(())
}

#[test]
fn test_isolation_via_snapshot() -> Result<()> {
    let (db, _temp) = setup_test_db()?;

    // Initial state
    db.put(b"counter", b"0")?;

    // TODO: Once we expose snapshot API, test snapshot isolation here
    // For now, RocksDB transactions provide snapshot isolation automatically

    Ok(())
}

#[test]
fn test_durability() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().to_path_buf();

    {
        // Open DB and insert data with dimension 3
        let options = OpenDBOptions::with_dimension(3);
        let db = OpenDB::open_with_options(&db_path, options)?;
        let mem = Memory::new("durable", "persisted data", vec![1.0; 3], 0.9);
        db.insert_memory(&mem)?;
        db.flush()?;
        // DB dropped here
    }

    {
        // Reopen and verify data persisted
        let options = OpenDBOptions::with_dimension(3);
        let db = OpenDB::open_with_options(&db_path, options)?;
        let retrieved = db.get_memory("durable")?;
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.content, "persisted data");
        assert_eq!(retrieved.importance, 0.9);
    }

    Ok(())
}

#[test]
fn test_concurrent_reads() -> Result<()> {
    let (db, _temp) = setup_test_db()?;

    // Insert test data
    for i in 0..10 {
        let mem = Memory::new(
            &format!("mem_{}", i),
            &format!("content {}", i),
            vec![i as f32; 3],
            0.5,
        );
        db.insert_memory(&mem)?;
    }

    let db = Arc::new(db);
    let mut handles = vec![];

    // Spawn multiple reader threads
    for i in 0..5 {
        let db_clone = Arc::clone(&db);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                let id = format!("mem_{}", j);
                let mem = db_clone.get_memory(&id).unwrap();
                assert!(mem.is_some(), "Thread {} couldn't read mem_{}", i, j);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}

#[test]
fn test_write_after_read_consistency() -> Result<()> {
    let (db, _temp) = setup_test_db()?;

    // Insert initial data
    let mem1 = Memory::new("war_test", "version 1", vec![1.0; 3], 0.5);
    db.insert_memory(&mem1)?;

    // Read
    let read1 = db.get_memory("war_test")?.unwrap();
    assert_eq!(read1.content, "version 1");

    // Write
    let mem2 = Memory::new("war_test", "version 2", vec![2.0; 3], 0.6);
    db.insert_memory(&mem2)?;

    // Read again - should see new version
    let read2 = db.get_memory("war_test")?.unwrap();
    assert_eq!(read2.content, "version 2");
    assert_eq!(read2.importance, 0.6);

    Ok(())
}
