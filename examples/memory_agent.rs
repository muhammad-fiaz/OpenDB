// Memory agent example
//
// Demonstrates using OpenDB as a memory backend for an AI agent.

use opendb::{OpenDB, Memory, OpenDBOptions, Result};

fn main() -> Result<()> {
    println!("=== Memory Agent Demo ===\n");

    // Open database with custom vector dimension (matching your embedding model)
    let options = OpenDBOptions::with_dimension(384);
    let db = OpenDB::open_with_options("./agent_memory_db", options)?;

    // Simulate agent interactions
    let interactions = vec![
        ("mem_001", "User mentioned they enjoy hiking", 0.7),
        ("mem_002", "User is a software engineer at TechCorp", 0.9),
        ("mem_003", "User prefers coffee over tea", 0.5),
        ("mem_004", "User is learning Rust programming", 0.95),
        ("mem_005", "User has a dog named Max", 0.6),
    ];

    // Insert memories with synthetic embeddings
    for (id, content, importance) in interactions {
        let embedding = generate_synthetic_embedding(content);
        let memory = Memory::new(id, content, embedding, importance)
            .with_metadata("type", "user_fact")
            .with_metadata("agent", "assistant_v1");
        
        db.insert_memory(&memory)?;
    }
    println!("✓ Stored {} agent memories", 5);

    // Create semantic relationships
    db.link("mem_002", "related_to", "mem_004")?; // Job relates to learning
    db.link("mem_001", "similar_to", "mem_005")?; // Outdoor person + pet
    println!("✓ Created semantic relationships");

    // Query: Find memories related to work
    let work_query = generate_synthetic_embedding("work and career");
    let work_memories = db.search_similar(&work_query, 3)?;
    
    println!("\n📋 Work-related memories:");
    for result in work_memories {
        println!("  - {} (relevance: {:.2})", 
                 result.memory.content, 
                 1.0 / (1.0 + result.distance));
    }

    // Query: Find memories about hobbies
    let hobby_query = generate_synthetic_embedding("hobbies and interests");
    let hobby_memories = db.search_similar(&hobby_query, 3)?;
    
    println!("\n🎯 Hobby-related memories:");
    for result in hobby_memories {
        println!("  - {} (relevance: {:.2})", 
                 result.memory.content, 
                 1.0 / (1.0 + result.distance));
    }

    // Get important memories (importance > 0.8)
    let all = db.list_memories("mem")?;
    let important: Vec<_> = all.into_iter()
        .filter(|m| m.importance > 0.8)
        .collect();
    
    println!("\n⭐ High-importance memories:");
    for mem in important {
        println!("  - {} (importance: {})", mem.content, mem.importance);
    }

    // Graph traversal
    let related_to_job = db.get_related("mem_002", "related_to")?;
    println!("\n🔗 Memories related to job:");
    for id in related_to_job {
        if let Some(mem) = db.get_memory(&id)? {
            println!("  - {}", mem.content);
        }
    }

    db.flush()?;
    println!("\n=== Demo Complete ===");
    Ok(())
}

/// Generate a synthetic embedding (in practice, use a real embedding model)
fn generate_synthetic_embedding(text: &str) -> Vec<f32> {
    // Simple hash-based synthetic embedding for demo
    let mut embedding = vec![0.0; 384];
    for (i, c) in text.chars().enumerate() {
        embedding[i % 384] += (c as u32 as f32) / 1000.0;
    }
    
    // Normalize
    let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in &mut embedding {
            *x /= norm;
        }
    }
    
    embedding
}
