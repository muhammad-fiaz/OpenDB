# Performance Tuning

This guide covers optimization strategies for OpenDB deployments.

## Profiling

Before optimizing, measure your bottleneck:

```rust
use std::time::Instant;

let start = Instant::now();
db.insert_memory(&memory)?;
println!("Insert took: {:?}", start.elapsed());
```

## RocksDB Tuning

### Write Buffer Size

Larger write buffers improve write throughput:

```rust
// Default: 128 MB
// For write-heavy workloads, increase:
opts.set_write_buffer_size(256 * 1024 * 1024); // 256 MB
```

**Trade-offs:**

- ✅ Fewer flushes to disk
- ✅ Better write throughput
- ❌ More memory usage
- ❌ Longer recovery time after crash

### Block Cache

RocksDB's internal cache for disk blocks:

```rust
opts.set_block_cache_size(512 * 1024 * 1024); // 512 MB
```

**Trade-offs:**

- ✅ Faster reads
- ❌ More memory usage

### Compression

Balance CPU vs storage:

```rust
use rocksdb::DBCompressionType;

// Default: LZ4 (fast, moderate compression)
opts.set_compression_type(DBCompressionType::Lz4);

// For better compression (slower writes):
opts.set_compression_type(DBCompressionType::Zstd);

// For faster writes (larger storage):
opts.set_compression_type(DBCompressionType::None);
```

### Parallelism

Increase background threads for compaction:

```rust
opts.increase_parallelism(4); // Use 4 threads
```

## Cache Tuning

### Cache Sizes

Adjust cache capacity based on workload:

```rust
use opendb::OpenDBOptions;

let mut options = OpenDBOptions::default();

// For read-heavy workloads
options.kv_cache_size = 10_000;
options.record_cache_size = 5_000;

// For write-heavy workloads (smaller cache)
options.kv_cache_size = 1_000;
options.record_cache_size = 500;

let db = OpenDB::open_with_options("./db", options)?;
```

### Cache Hit Rate

Monitor cache effectiveness:

```rust
// Implement hit rate tracking (example)
struct CacheStats {
    hits: AtomicU64,
    misses: AtomicU64,
}

impl CacheStats {
    fn hit_rate(&self) -> f64 {
        let hits = self.hits.load(Ordering::Relaxed) as f64;
        let misses = self.misses.load(Ordering::Relaxed) as f64;
        hits / (hits + misses)
    }
}
```

**Target hit rates:**

- **> 90%:** Excellent
- **70-90%:** Good
- **< 70%:** Increase cache size

## Batch Operations

### Batch Inserts

Use transactions for bulk inserts:

```rust
// ❌ Slow: Individual commits
for memory in memories {
    db.insert_memory(&memory)?;
}

// ✅ Fast: Batch commit (future API)
let mut txn = db.begin_transaction()?;
for memory in memories {
    // Insert via transaction
}
txn.commit()?;
```

### Flush Control

Control when data is flushed to disk:

```rust
// Insert many records
for i in 0..10_000 {
    db.insert_memory(&memory)?;
}

// Explicit flush
db.flush()?;
```

## Vector Search Optimization

### Index Parameters

Tune HNSW parameters for your use case:

```rust
// High accuracy (slower, better recall)
HnswParams::high_accuracy()  // ef=400, neighbors=32

// High speed (faster, lower recall)
HnswParams::high_speed()     // ef=100, neighbors=8
```

### Rebuild Strategy

Rebuild index strategically:

```rust
// ❌ Bad: Rebuild after every insert
for memory in memories {
    db.insert_memory(&memory)?;
    db.rebuild_vector_index()?; // Expensive!
}

// ✅ Good: Rebuild once after batch
for memory in memories {
    db.insert_memory(&memory)?;
}
db.rebuild_vector_index()?; // Once
```

### Dimension Reduction

Lower dimensions = faster search:

```rust
// 768D (high quality, slower)
options.vector_dimension = 768;

// 384D (balanced)
options.vector_dimension = 384;

// 128D (fast, lower quality)
options.vector_dimension = 128;
```

## Graph Optimization

### Link Batching

Batch graph operations:

```rust
// Create all memories first
for memory in memories {
    db.insert_memory(&memory)?;
}

// Then create all links
for (from, to, relation) in edges {
    db.link(from, to, relation)?;
}
```

### Prune Unused Relations

Remove stale edges periodically:

```rust
fn prune_orphaned_edges(db: &OpenDB) -> Result<()> {
    let all_ids: HashSet<_> = db.list_memory_ids()?.into_iter().collect();
    
    for id in db.list_memory_ids()? {
        let outgoing = db.get_outgoing(&id)?;
        for edge in outgoing {
            if !all_ids.contains(&edge.to) {
                db.unlink(&edge.from, &edge.to, &edge.relation)?;
            }
        }
    }
    
    Ok(())
}
```

## Memory Usage

### Estimate Memory Footprint

```
Total Memory = 
    RocksDB Write Buffers +
    RocksDB Block Cache +
    Application Caches +
    HNSW Index +
    Overhead

Example:
    128 MB (write buffers) +
    256 MB (block cache) +
    10 MB (app caches, 10k entries × 1KB avg) +
    30 MB (HNSW, 10k vectors × 384D × 4 bytes × 2x overhead) +
    50 MB (overhead)
    = ~474 MB
```

### Reduce Memory Usage

1. **Smaller caches:**

```rust
options.kv_cache_size = 100;
options.record_cache_size = 100;
```

2. **Lower RocksDB buffers:**

```rust
opts.set_write_buffer_size(64 * 1024 * 1024); // 64 MB
opts.set_block_cache_size(128 * 1024 * 1024); // 128 MB
```

3. **Smaller embeddings:**

```rust
options.vector_dimension = 128; // Instead of 768
```

## Disk Usage

### Compaction

Force compaction to reclaim space:

```rust
// Manual compaction (future API)
db.compact_range(None, None)?;
```

### Monitoring

Check database size:

```rust
// On Linux
std::process::Command::new("du")
    .args(&["-sh", "./db"])
    .output()?;
```

## Benchmarking

Use Criterion for accurate benchmarks:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_insert(c: &mut Criterion) {
    let db = OpenDB::open("./bench_db").unwrap();
    
    c.bench_function("insert_memory", |b| {
        b.iter(|| {
            let memory = Memory::new("id".to_string(), "content".to_string());
            db.insert_memory(black_box(&memory)).unwrap();
        });
    });
}

criterion_group!(benches, benchmark_insert);
criterion_main!(benches);
```

## Monitoring Metrics

Implement metrics collection:

```rust
struct Metrics {
    reads: AtomicU64,
    writes: AtomicU64,
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
}

impl Metrics {
    fn report(&self) {
        println!("Reads: {}", self.reads.load(Ordering::Relaxed));
        println!("Writes: {}", self.writes.load(Ordering::Relaxed));
        println!("Cache hit rate: {:.2}%", 
            self.cache_hits.load(Ordering::Relaxed) as f64 /
            (self.cache_hits.load(Ordering::Relaxed) + 
             self.cache_misses.load(Ordering::Relaxed)) as f64 * 100.0
        );
    }
}
```

## Platform-Specific Tips

### Linux

- Use `io_uring` for async I/O (future RocksDB feature)
- Disable transparent huge pages for lower latency
- Use `fallocate` for preallocating disk space

### macOS

- APFS filesystem has good performance
- Use `F_NOCACHE` for large scans (avoid cache pollution)

### Windows

- Use NTFS for best RocksDB performance
- Disable indexing on database directory
- Use SSD for best performance

## Common Bottlenecks

1. **Slow writes:** Increase write buffer size, disable compression
2. **Slow reads:** Increase cache sizes, use SSD
3. **High memory:** Reduce cache sizes, lower embedding dimension
4. **Slow vector search:** Reduce HNSW parameters, lower dimension
5. **Large database size:** Enable compression, run compaction

## Next

- [Extending OpenDB](extending.md)
- [Contributing](../contributing.md)
