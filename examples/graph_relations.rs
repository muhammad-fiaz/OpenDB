// Graph relations example

use opendb::{Memory, OpenDB, Result};

fn main() -> Result<()> {
    println!("=== Graph Relations Demo ===\n");

    let db = OpenDB::open("./graph_demo_db")?;

    // Create a knowledge graph of concepts
    let concepts = vec![
        (
            "rust",
            "Rust is a systems programming language",
            vec![0.1; 4],
        ),
        (
            "memory_safety",
            "Memory safety without garbage collection",
            vec![0.2; 4],
        ),
        (
            "ownership",
            "Ownership system prevents data races",
            vec![0.3; 4],
        ),
        (
            "borrowing",
            "Borrowing allows references to data",
            vec![0.4; 4],
        ),
        (
            "lifetimes",
            "Lifetimes ensure references are valid",
            vec![0.5; 4],
        ),
    ];

    // Insert concepts
    for (id, content, embedding) in concepts {
        let memory = Memory::new(id, content, embedding, 0.8);
        db.insert_memory(&memory)?;
    }
    println!("✓ Created knowledge graph with 5 concepts");

    // Build the concept graph
    db.link("rust", "has_feature", "memory_safety")?;
    db.link("rust", "has_feature", "ownership")?;
    db.link("ownership", "enables", "memory_safety")?;
    db.link("ownership", "includes", "borrowing")?;
    db.link("borrowing", "requires", "lifetimes")?;
    db.link("memory_safety", "prevents", "rust")?; // Circular reference
    println!("✓ Built concept relationships");

    // Traverse: What features does Rust have?
    println!("\n🔍 Features of Rust:");
    let features = db.get_related("rust", "has_feature")?;
    for feature_id in features {
        if let Some(mem) = db.get_memory(&feature_id)? {
            println!("  - {}: {}", feature_id, mem.content);
        }
    }

    // Traverse: What does ownership enable?
    println!("\n🔍 What ownership enables:");
    let enabled = db.get_related("ownership", "enables")?;
    for id in enabled {
        if let Some(mem) = db.get_memory(&id)? {
            println!("  - {}", mem.content);
        }
    }

    // Get all outgoing edges from ownership
    println!("\n🔍 All relationships from 'ownership':");
    let edges = db.get_outgoing("ownership")?;
    for edge in edges {
        println!("  - {} --[{}]--> {}", edge.from, edge.relation, edge.to);
    }

    // Get all incoming edges to memory_safety
    println!("\n🔍 All relationships to 'memory_safety':");
    let incoming = db.get_incoming("memory_safety")?;
    for edge in incoming {
        println!("  - {} --[{}]--> {}", edge.from, edge.relation, edge.to);
    }

    // Bidirectional traversal
    println!("\n🔗 Full graph structure:");
    let all_concepts = db.list_memory_ids("").unwrap_or_default();
    for concept in all_concepts {
        let outgoing = db.get_outgoing(&concept)?;
        if !outgoing.is_empty() {
            println!("\n  {}:", concept);
            for edge in outgoing {
                println!("    → {} ({})", edge.to, edge.relation);
            }
        }
    }

    db.flush()?;
    println!("\n=== Graph Demo Complete ===");
    Ok(())
}
