# Vector Search API

OpenDB provides semantic similarity search using HNSW (Hierarchical Navigable Small World) index.

## Overview

Vector search enables finding memories based on semantic similarity rather than exact matches:

```rust
use opendb::OpenDB;

let db = OpenDB::open("./db")?;

// Insert memories with embeddings
let memory = Memory {
    id: "mem_001".to_string(),
    content: "Rust is a systems programming language".to_string(),
    embedding: generate_embedding("Rust is a systems programming language"),
    ..Default::default()
};
db.insert_memory(&memory)?;

// Search by query embedding
let query_embedding = generate_embedding("What is Rust?");
let results = db.search_similar(&query_embedding, 5)?;
```

## Search Similar

Find memories similar to a query vector:

```rust
let results = db.search_similar(&query_embedding, top_k)?;

for result in results {
    println!("ID: {}, Distance: {}", result.id, result.distance);
    let memory = db.get_memory(&result.id)?.unwrap();
    println!("Content: {}", memory.content);
}
```

**Signature:**

```rust
pub fn search_similar(&self, query: &[f32], top_k: usize) -> Result<Vec<SearchResult>>
```

**Parameters:**

- `query`: Query vector (must match configured dimension)
- `top_k`: Number of results to return

**Returns:** `Vec<SearchResult>` sorted by distance (closest first).

### SearchResult Type

```rust
pub struct SearchResult {
    pub id: String,
    pub distance: f32,
}
```

- **id**: Memory ID
- **distance**: Euclidean distance (lower = more similar)

## Embeddings

### Dimension Configuration

Set embedding dimension when opening database:

```rust
use opendb::OpenDBOptions;

let mut options = OpenDBOptions::default();
options.vector_dimension = 768; // For OpenAI ada-002 or similar
let db = OpenDB::open_with_options("./db", options)?;
```

**Default:** 384 (for sentence-transformers/all-MiniLM-L6-v2)

### Generating Embeddings

OpenDB does **not** include embedding generation. Use external models:

#### Example: sentence-transformers (Python)

```python
from sentence_transformers import SentenceTransformer

model = SentenceTransformer('all-MiniLM-L6-v2')
embedding = model.encode("Hello world").tolist()  # [0.1, -0.2, ...]
```

#### Example: OpenAI API

```rust
// Pseudo-code (use openai-rust crate)
let embedding = openai_client
    .embeddings("text-embedding-ada-002")
    .create("Hello world")
    .await?;
```

#### Example: Candle (Rust)

```rust
// Use candle-transformers for local inference
// See: https://github.com/huggingface/candle
```

### Synthetic Embeddings (Testing)

For testing without real models:

```rust
fn generate_synthetic_embedding(text: &str, dimension: usize) -> Vec<f32> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    let seed = hasher.finish();
    
    let mut rng = /* initialize with seed */;
    (0..dimension).map(|_| rng.gen_range(-1.0..1.0)).collect()
}
```

## Index Management

### Automatic Index Building

The HNSW index is built automatically on first search:

```rust
// Insert memories
db.insert_memory(&memory1)?;
db.insert_memory(&memory2)?;

// First search triggers index build
let results = db.search_similar(&query, 5)?; // Builds index here
```

### Manual Rebuild

Force index rebuild (e.g., after bulk inserts):

```rust
db.rebuild_vector_index()?;
```

**Signature:**

```rust
pub fn rebuild_vector_index(&self) -> Result<()>
```

**When to rebuild:**

- After bulk memory inserts
- After changing embeddings
- To incorporate deleted memories

**Note:** Search automatically rebuilds if index is stale.

## HNSW Parameters

HNSW has tunable parameters for speed vs accuracy tradeoff:

### Default Parameters

```rust
pub struct HnswParams {
    pub ef_construction: usize, // 200
    pub max_neighbors: usize,   // 16
}
```

### Presets

```rust
// High accuracy (slower build, better recall)
HnswParams::high_accuracy()  // ef=400, neighbors=32

// High speed (faster build, lower recall)
HnswParams::high_speed()     // ef=100, neighbors=8

// Balanced (default)
HnswParams::default()        // ef=200, neighbors=16
```

**Note:** Currently not exposed in OpenDB API. Future versions will allow tuning.

## Distance Metric

OpenDB uses **Euclidean distance**:

$$
d(p, q) = \sqrt{\sum_{i=1}^{n} (p_i - q_i)^2}
$$

**Properties:**

- Lower distance = more similar
- Distance 0 = identical vectors
- Sensitive to magnitude (normalize if needed)

### Normalization

For cosine similarity behavior, normalize embeddings:

```rust
fn normalize(vec: &mut Vec<f32>) {
    let magnitude: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
    for x in vec.iter_mut() {
        *x /= magnitude;
    }
}

let mut embedding = generate_embedding(text);
normalize(&mut embedding);
```

## Usage Patterns

### Semantic Memory Search

```rust
// User asks a question
let query = "How do I prevent memory leaks in Rust?";
let query_embedding = generate_embedding(query);

// Find relevant memories
let results = db.search_similar(&query_embedding, 3)?;
for result in results {
    let memory = db.get_memory(&result.id)?.unwrap();
    println!("Relevant memory: {}", memory.content);
}
```

### Deduplication

Find duplicate or near-duplicate content:

```rust
let new_content = "Rust ownership prevents data races";
let new_embedding = generate_embedding(new_content);

let similar = db.search_similar(&new_embedding, 1)?;
if let Some(top) = similar.first() {
    if top.distance < 0.1 {  // Threshold for "duplicate"
        println!("Similar content already exists: {}", top.id);
    }
}
```

### Clustering

Group similar memories:

```rust
let all_memories = db.list_memories()?;
let mut clusters: Vec<Vec<String>> = Vec::new();

for memory in all_memories {
    if memory.embedding.is_empty() {
        continue;
    }
    
    let similar = db.search_similar(&memory.embedding, 5)?;
    let cluster: Vec<String> = similar.iter()
        .filter(|r| r.distance < 0.5)  // Similarity threshold
        .map(|r| r.id.clone())
        .collect();
    
    clusters.push(cluster);
}
```

## Performance Characteristics

| Operation | Time Complexity | Typical Latency |
|-----------|----------------|-----------------|
| `search_similar()` | O(log n) | ~1-10ms |
| `rebuild_vector_index()` | O(n log n) | ~100ms per 1k vectors |
| Insert with embedding | O(1) + rebuild | Instant (rebuild deferred) |

**Scalability:**

- **100-1k memories:** Instant search
- **1k-10k memories:** <10ms search
- **10k-100k memories:** <50ms search
- **100k+ memories:** Consider sharding (future feature)

## Limitations

1. **Dimension Mismatch:** All embeddings must have same dimension
2. **No Incremental Updates:** Index rebuild is full reconstruction
3. **Memory Usage:** HNSW index kept in memory (~4 bytes × dimension × count)
4. **No GPU Support:** Pure CPU implementation

## Error Handling

```rust
use opendb::Error;

match db.search_similar(&query, 10) {
    Ok(results) => { /* use results */ },
    Err(Error::VectorIndex(e)) => println!("Index error: {}", e),
    Err(Error::InvalidInput(e)) => println!("Bad query: {}", e),
    Err(e) => println!("Other error: {}", e),
}
```

## Best Practices

1. **Batch Inserts:** Insert all memories, then rebuild once:

```rust
for memory in memories {
    db.insert_memory(&memory)?;
}
db.rebuild_vector_index()?; // One rebuild for all
```

2. **Lazy Embeddings:** Only generate embeddings for searchable content:

```rust
let memory = Memory::new(id, content);
// Don't set embedding if this memory won't be searched
db.insert_memory(&memory)?;
```

3. **Relevance Filtering:** Filter by distance threshold:

```rust
let results = db.search_similar(&query, 20)?;
let relevant: Vec<_> = results.into_iter()
    .filter(|r| r.distance < 1.0)  // Adjust threshold
    .collect();
```

4. **Combine with Metadata:** Use metadata to post-filter:

```rust
let results = db.search_similar(&query, 50)?;
for result in results {
    let memory = db.get_memory(&result.id)?.unwrap();
    if memory.metadata.get("category") == Some(&"docs".to_string()) {
        println!("Relevant doc: {}", memory.content);
    }
}
```

## Next

- [Transactions API](transactions.md)
- [Performance Tuning](../advanced/performance.md)
