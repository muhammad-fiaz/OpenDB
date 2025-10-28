# Architecture Overview

OpenDB is designed as a modular, hybrid database system that combines multiple database paradigms while maintaining high performance and ACID guarantees.

## System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  OpenDB Public API                       │
├─────────────┬──────────────┬──────────────┬─────────────┤
│  Key-Value  │   Records    │    Graph     │   Vectors   │
│   Store     │  (Memory)    │  Relations   │   (HNSW)    │
├─────────────┴──────────────┴──────────────┴─────────────┤
│           Transaction Manager (ACID)                     │
│        WAL + Optimistic Locking + MVCC                  │
├──────────────────────────────────────────────────────────┤
│              LRU Cache Layer                             │
│        (Write-Through + Invalidation)                   │
├──────────────────────────────────────────────────────────┤
│         Storage Trait (Pluggable Backend)                │
├──────────────────────────────────────────────────────────┤
│            RocksDB Backend (LSM Tree)                    │
│    Column Families + Native Transactions + WAL          │
└──────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Storage Layer

- **Backend**: RocksDB (high-performance LSM tree)
- **Column Families**: Namespace isolation for different data types
- **Persistence**: Write-Ahead Log (WAL) for durability

### 2. Transaction Manager

- **ACID Guarantees**: Full transactional support
- **Isolation**: Snapshot isolation via RocksDB transactions
- **Concurrency**: Optimistic locking

### 3. Cache Layer

- **Strategy**: LRU (Least Recently Used)
- **Write Policy**: Write-through (update storage first, then cache)
- **Coherency**: Automatic invalidation on delete

### 4. Feature Modules

#### Key-Value Store
- Direct byte-level storage
- Prefix scans
- Cache-accelerated reads

#### Records Manager
- Structured Memory records
- Codec: rkyv (zero-copy deserialization)
- Metadata support

#### Graph Manager
- Bidirectional adjacency lists
- Forward index: `from → [(relation, to)]`
- Backward index: `to → [(relation, from)]`

#### Vector Manager
- HNSW index for approximate nearest neighbor search
- Automatic index rebuilding
- Configurable search quality

## Data Flow

### Write Path

```
Application → OpenDB API → Cache (update) → Storage Backend → WAL → Disk
```

### Read Path (Cache Hit)

```
Application → OpenDB API → Cache → Return
```

### Read Path (Cache Miss)

```
Application → OpenDB API → Cache (miss) → Storage Backend → Cache (populate) → Return
```

## Design Decisions

### Why RocksDB?

**Advantages:**
- Production-tested LSM tree
- Excellent write throughput
- Built-in WAL and transactions
- Column families for organization

**Tradeoffs:**
- Not pure Rust (C++ with bindings)
- Larger binary size

**Alternatives Considered:**
- `redb`: Pure Rust, B-tree based, simpler but lower throughput
- `sled`: Pure Rust, but less mature and maintenance concerns
- Custom LSM: Too much complexity for initial version

### Why rkyv for Serialization?

**Advantages:**
- Zero-copy deserialization (fast reads)
- Schema versioning support
- Type safety

**Alternatives:**
- `bincode`: Simpler but requires full deserialization
- `serde_json`: Human-readable but slower

### Why HNSW for Vector Search?

**Advantages:**
- Excellent accuracy/speed tradeoff
- Logarithmic search complexity
- Works well for high-dimensional data

**Alternatives:**
- IVF (Inverted File Index): Faster but less accurate
- Flat index: Exact but O(n) search

## Next Steps

- [Storage Layer Details](storage.md)
- [Transaction Model](transactions.md)
- [Caching Strategy](caching.md)
