// Quick start example for OpenDB
//
// This demonstrates basic usage of OpenDB for agent memory storage.

use opendb::{OpenDB, Memory, Result};

fn main() -> Result<()> {
    println!("=== OpenDB Quickstart ===\n");

    // Open or create a database
    let db = OpenDB::open("./quickstart_db")?;
    println!("✓ Database opened");

    // Create some memory records
    let memory1 = Memory::new(
        "memory_001",
        "The user prefers dark mode for coding",
        vec![0.1, 0.2, 0.3, 0.4],
        0.9,
    )
    .with_metadata("category", "preference")
    .with_metadata("source", "user_settings");

    let memory2 = Memory::new(
        "memory_002",
        "The user is working on a Rust database project",
        vec![0.15, 0.25, 0.35, 0.45],
        0.95,
    )
    .with_metadata("category", "context")
    .with_metadata("project", "opendb");

    let memory3 = Memory::new(
        "memory_003",
        "The user asked about ACID transaction support",
        vec![0.12, 0.22, 0.32, 0.42],
        0.8,
    )
    .with_metadata("category", "question");

    // Insert memories
    db.insert_memory(&memory1)?;
    db.insert_memory(&memory2)?;
    db.insert_memory(&memory3)?;
    println!("✓ Inserted 3 memories");

    // Retrieve a memory
    if let Some(retrieved) = db.get_memory("memory_001")? {
        println!("\n✓ Retrieved memory: {}", retrieved.content);
        println!("  Importance: {}", retrieved.importance);
        println!("  Metadata: {:?}", retrieved.metadata);
    }

    // Create relationships
    db.link("memory_002", "related_to", "memory_001")?;
    db.link("memory_003", "references", "memory_002")?;
    println!("\n✓ Created relationships");

    // Query relationships
    let related = db.get_related("memory_002", "related_to")?;
    println!("  memory_002 is related to: {:?}", related);

    // Vector search - find similar memories
    let query = vec![0.1, 0.2, 0.3, 0.4];
    let similar = db.search_similar(&query, 2)?;
    println!("\n✓ Vector search results:");
    for result in similar {
        println!("  - {} (distance: {:.4})", result.memory.content, result.distance);
    }

    // List all memories
    let all_memories = db.list_memories("memory")?;
    println!("\n✓ Total memories: {}", all_memories.len());

    // Key-value operations
    db.put(b"config:theme", b"dark")?;
    if let Some(theme) = db.get(b"config:theme")? {
        println!("\n✓ KV store: theme = {}", String::from_utf8_lossy(&theme));
    }

    // Flush to disk
    db.flush()?;
    println!("\n✓ All changes flushed to disk");

    println!("\n=== Quickstart Complete ===");
    Ok(())
}
