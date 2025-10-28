# Graph API

OpenDB provides a labeled property graph for modeling relationships between memories.

## Core Concepts

- **Nodes:** `Memory` objects (referenced by ID)
- **Edges:** Directed relationships with labels and weights
- **Relations:** String labels like `"causes"`, `"before"`, `"similar_to"`

## Edge Type

```rust
pub struct Edge {
    pub from: String,
    pub relation: String,
    pub to: String,
    pub weight: f64,
    pub timestamp: i64,
}
```

## Linking Memories

### Basic Link

```rust
use opendb::OpenDB;

let db = OpenDB::open("./db")?;

// Create two memories
let mem1 = Memory::new("mem_001".to_string(), "Rust is fast".to_string());
let mem2 = Memory::new("mem_002".to_string(), "C++ is fast".to_string());
db.insert_memory(&mem1)?;
db.insert_memory(&mem2)?;

// Link them
db.link("mem_001", "mem_002", "similar_to")?;
```

**Signature:**

```rust
pub fn link(&self, from: &str, to: &str, relation: &str) -> Result<()>
```

**Behavior:**

- Creates directed edge from `from` → `to`
- Default weight: 1.0
- Stores in both forward and backward indexes
- Allows multiple relations between same nodes

### Custom Weight

```rust
use opendb::{OpenDB, Edge};

let edge = Edge {
    from: "mem_001".to_string(),
    relation: "causes".to_string(),
    to: "mem_002".to_string(),
    weight: 0.85,  // Custom confidence score
    timestamp: chrono::Utc::now().timestamp(),
};

// Link via graph manager (internal API, use link() for simple cases)
```

## Unlinking

Remove a specific relationship:

```rust
db.unlink("mem_001", "mem_002", "similar_to")?;
```

**Signature:**

```rust
pub fn unlink(&self, from: &str, to: &str, relation: &str) -> Result<()>
```

**Behavior:**

- Removes edge from both indexes
- Succeeds even if edge doesn't exist
- Does **not** delete the nodes

## Querying Relationships

### Get All Related Nodes

```rust
let related = db.get_related("mem_001", "similar_to")?;
for edge in related {
    println!("{} --[{}]--> {} (weight: {})", 
        edge.from, edge.relation, edge.to, edge.weight);
}
```

**Signature:**

```rust
pub fn get_related(&self, id: &str, relation: &str) -> Result<Vec<Edge>>
```

**Returns:** All edges from `id` with the specified relation.

### Get Outgoing Edges

```rust
let outgoing = db.get_outgoing("mem_001")?;
for edge in outgoing {
    println!("Outgoing: {} --[{}]--> {}", edge.from, edge.relation, edge.to);
}
```

**Signature:**

```rust
pub fn get_outgoing(&self, id: &str) -> Result<Vec<Edge>>
```

**Returns:** All edges where `id` is the source (all relations).

### Get Incoming Edges

```rust
let incoming = db.get_incoming("mem_002")?;
for edge in incoming {
    println!("Incoming: {} --[{}]--> {}", edge.from, edge.relation, edge.to);
}
```

**Signature:**

```rust
pub fn get_incoming(&self, id: &str) -> Result<Vec<Edge>>
```

**Returns:** All edges where `id` is the target (all relations).

## Relation Types

OpenDB provides predefined relation constants:

```rust
pub mod relation {
    pub const RELATED_TO: &str = "related_to";
    pub const CAUSED_BY: &str = "caused_by";
    pub const BEFORE: &str = "before";
    pub const AFTER: &str = "after";
    pub const REFERENCES: &str = "references";
    pub const SIMILAR_TO: &str = "similar_to";
    pub const CONTRADICTS: &str = "contradicts";
    pub const SUPPORTS: &str = "supports";
}
```

### Usage

```rust
use opendb::graph::relation;

db.link("mem_001", "mem_002", relation::CAUSED_BY)?;
db.link("mem_002", "mem_003", relation::BEFORE)?;
```

### Custom Relations

You can use any string as a relation:

```rust
db.link("mem_001", "mem_002", "depends_on")?;
db.link("mem_003", "mem_004", "implements")?;
```

## Graph Patterns

### Temporal Chain

```rust
use opendb::graph::relation;

// Build timeline
db.link("event_1", "event_2", relation::BEFORE)?;
db.link("event_2", "event_3", relation::BEFORE)?;
db.link("event_3", "event_4", relation::BEFORE)?;

// Traverse forward
let next_events = db.get_related("event_1", relation::BEFORE)?;
```

### Causal Graph

```rust
use opendb::graph::relation;

// A causes B, B causes C
db.link("symptom_A", "symptom_B", relation::CAUSED_BY)?;
db.link("symptom_B", "symptom_C", relation::CAUSED_BY)?;

// Find root causes
let causes = db.get_incoming("symptom_C")?;
```

### Knowledge Graph

```rust
use opendb::graph::relation;

// Rust has ownership
db.link("rust", "ownership", "has_feature")?;
// Ownership enables memory_safety
db.link("ownership", "memory_safety", "enables")?;
// Memory_safety prevents bugs
db.link("memory_safety", "bug_prevention", "prevents")?;

// Traverse features
let features = db.get_related("rust", "has_feature")?;
```

### Bidirectional Relationships

```rust
// A is similar to B
db.link("mem_A", "mem_B", "similar_to")?;
// B is also similar to A
db.link("mem_B", "mem_A", "similar_to")?;

// Query either direction
let similar_from_A = db.get_related("mem_A", "similar_to")?;
let similar_from_B = db.get_related("mem_B", "similar_to")?;
```

## Advanced Queries

### Multi-Hop Traversal

```rust
fn traverse_depth_2(db: &OpenDB, start: &str, relation: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();
    
    // First hop
    let hop1 = db.get_related(start, relation)?;
    for edge1 in hop1 {
        result.push(edge1.to.clone());
        
        // Second hop
        let hop2 = db.get_related(&edge1.to, relation)?;
        for edge2 in hop2 {
            result.push(edge2.to.clone());
        }
    }
    
    Ok(result)
}
```

### Filter by Weight

```rust
let edges = db.get_related("mem_001", "similar_to")?;
let strong_edges: Vec<_> = edges.into_iter()
    .filter(|e| e.weight > 0.8)
    .collect();
```

### Aggregate Relations

```rust
use std::collections::HashMap;

let outgoing = db.get_outgoing("mem_001")?;
let mut relation_counts: HashMap<String, usize> = HashMap::new();

for edge in outgoing {
    *relation_counts.entry(edge.relation).or_insert(0) += 1;
}

println!("Relation distribution: {:?}", relation_counts);
```

## Performance Characteristics

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| `link()` | O(log n) | Two index writes (forward + backward) |
| `unlink()` | O(k log n) | k = edges between nodes |
| `get_related()` | O(log n + k) | k = matching edges |
| `get_outgoing()` | O(log n + k) | k = total outgoing edges |
| `get_incoming()` | O(log n + k) | k = total incoming edges |

## Storage Details

Edges are stored in two column families:

1. **graph_forward:** `{from}:{relation}` → `Vec<Edge>`
2. **graph_backward:** `{to}:{relation}` → `Vec<Edge>`

This dual-indexing enables fast queries in both directions.

## Error Handling

```rust
use opendb::Error;

match db.link("mem_001", "mem_002", "related_to") {
    Ok(_) => println!("Link created"),
    Err(Error::Storage(_)) => println!("Storage error"),
    Err(Error::Graph(_)) => println!("Graph error"),
    Err(e) => println!("Other error: {}", e),
}
```

## Next

- [Vector API](vector.md)
- [Transactions](transactions.md)
- [Architecture: Transactions](../architecture/transactions.md)
