# Storage Layer

## RocksDB Backend

OpenDB uses RocksDB as its default storage backend, providing a robust foundation for ACID transactions and high-performance data access.

## Column Families

Data is organized into separate column families (namespaces):

| Column Family | Purpose | Data Format |
|--------------|---------|-------------|
| `default` | Key-value store | Raw bytes |
| `records` | Memory records | rkyv-encoded Memory structs |
| `graph_forward` | Forward adjacency list | rkyv-encoded Edge arrays |
| `graph_backward` | Backward adjacency list | rkyv-encoded Edge arrays |
| `vector_data` | Vector embeddings | bincode-encoded f32 arrays |
| `vector_index` | HNSW metadata | (currently in-memory) |
| `metadata` | DB metadata | JSON |

## Storage Trait

The storage layer is abstracted behind a trait, allowing for pluggable backends:

```rust
pub trait StorageBackend: Send + Sync {
    fn get(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>>;
    fn put(&self, cf: &str, key: &[u8], value: &[u8]) -> Result<()>;
    fn delete(&self, cf: &str, key: &[u8]) -> Result<()>;
    fn scan_prefix(&self, cf: &str, prefix: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>>;
    fn begin_transaction(&self) -> Result<Box<dyn Transaction>>;
    fn flush(&self) -> Result<()>;
}
```

## Performance Tuning

RocksDB is configured with optimizations for mixed read/write workloads:

```rust
// Write buffer: 128MB
opts.set_write_buffer_size(128 * 1024 * 1024);

// Number of write buffers: 3
opts.set_max_write_buffer_number(3);

// Target file size: 64MB
opts.set_target_file_size_base(64 * 1024 * 1024);

// Compression: LZ4
opts.set_compression_type(rocksdb::DBCompressionType::Lz4);
```

## Write-Ahead Log (WAL)

RocksDB's WAL ensures durability:

1. All writes are first appended to the WAL
2. Then applied to memtables
3. Periodically flushed to SST files
4. Old WAL segments are deleted after checkpoint

## LSM Tree Structure

RocksDB uses a Log-Structured Merge (LSM) tree:

```
Write Path:
  Write → WAL → MemTable → (flush) → L0 SST → (compact) → L1 SST → ...

Read Path:
  Read → MemTable → Block Cache → L0 → L1 → ... → Ln
```

### Advantages

- **Write Amplification**: Minimized for sequential writes
- **Compression**: Data is compressed at each level
- **Compaction**: Background process merges and cleans data

### Tradeoffs

- **Read Amplification**: May need to check multiple levels
- **Space Amplification**: Compaction creates temporary overhead

## Future Backend Options

### redb (Pure Rust B-Tree)

**Pros:**
- Pure Rust, no C++ dependencies
- Simpler architecture
- Good for read-heavy workloads

**Cons:**
- Lower write throughput than LSM
- Less mature

### Custom LSM Implementation

**Pros:**
- Full control over optimization
- Pure Rust

**Cons:**
- High development and maintenance cost
- Risk of bugs in critical path

## Next

- [Transaction Model](transactions.md)
- [Caching Strategy](caching.md)
