<div align="center">

# 🚀 OpenDB

### High-Performance Hybrid Embedded Database for Rust

[![Crates.io](https://img.shields.io/crates/v/opendb.svg)](https://crates.io/crates/opendb)
[![Documentation](https://docs.rs/opendb/badge.svg)](https://muhammad-fiaz.github.io/opendb)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://github.com/muhammad-fiaz/opendb#license)
[![Rust](https://img.shields.io/badge/rust-2021%2B-orange.svg)](https://www.rust-lang.org/)

[![CI](https://github.com/muhammad-fiaz/OpenDB/workflows/CI/badge.svg)](https://github.com/muhammad-fiaz/OpenDB/actions/workflows/ci.yml)
[![Deploy Docs](https://github.com/muhammad-fiaz/OpenDB/workflows/Deploy%20Documentation/badge.svg)](https://github.com/muhammad-fiaz/OpenDB/actions/workflows/docs-deploy.yml)
[![Publish](https://github.com/muhammad-fiaz/OpenDB/workflows/Publish%20to%20crates.io/badge.svg)](https://github.com/muhammad-fiaz/OpenDB/actions/workflows/publish.yml)

[![Downloads](https://img.shields.io/crates/d/opendb.svg)](https://crates.io/crates/opendb)
[![GitHub Stars](https://img.shields.io/github/stars/muhammad-fiaz/opendb?style=social)](https://github.com/muhammad-fiaz/opendb/stargazers)
[![GitHub Forks](https://img.shields.io/github/forks/muhammad-fiaz/opendb?style=social)](https://github.com/muhammad-fiaz/opendb/network/members)

[![Issues](https://img.shields.io/github/issues/muhammad-fiaz/opendb)](https://github.com/muhammad-fiaz/OpenDB/issues)
[![Pull Requests](https://img.shields.io/github/issues-pr/muhammad-fiaz/opendb)](https://github.com/muhammad-fiaz/OpenDB/pulls)
[![Last Commit](https://img.shields.io/github/last-commit/muhammad-fiaz/opendb)](https://github.com/muhammad-fiaz/opendb/commits/main)
[![Contributors](https://img.shields.io/github/contributors/muhammad-fiaz/opendb)](https://github.com/muhammad-fiaz/opendb/graphs/contributors)

[![Lines of Code](https://img.shields.io/tokei/lines/github/muhammad-fiaz/opendb)](https://github.com/muhammad-fiaz/opendb)
[![Code Size](https://img.shields.io/github/languages/code-size/muhammad-fiaz/opendb)](https://github.com/muhammad-fiaz/opendb)
[![Repo Size](https://img.shields.io/github/repo-size/muhammad-fiaz/opendb)](https://github.com/muhammad-fiaz/opendb)

---

</div>

**OpenDB** is a high-performance, pure Rust hybrid embedded database combining:

- **Key-Value Store**: Fast, persistent KV operations
- **Structured Records**: Schema-free JSON-like memory storage
- **Graph Relationships**: Bidirectional edges with typed relations
- **Vector Embeddings**: Semantic similarity search
- **ACID Transactions**: Full transaction support with snapshot isolation
- **In-Memory Caching**: LRU cache for hot data
- **Multimodal Support**: PDF, DOCX, audio, video, and text file processing for AI/LLM applications

Built on RocksDB for exceptional throughput and performance, OpenDB is designed for AI agent memory systems, knowledge graphs, semantic search, and multimodal RAG (Retrieval Augmented Generation) applications.

## Features

- 🚀 **High Performance**: Built on RocksDB with optimized LSM tree storage
- 🔒 **ACID Transactions**: Full transactional support with snapshot isolation
- 🧠 **Semantic Search**: Vector similarity search for embeddings
- 📊 **Graph Database**: Bidirectional relationship tracking with typed edges
- 💾 **Persistent Storage**: Durable RocksDB backend with write-ahead logging
- ⚡ **In-Memory Cache**: LRU caching for frequently accessed data
- 🔧 **Zero-Copy Serialization**: Fast encoding/decoding with rkyv
- 🦀 **Pure Rust**: Memory-safe, concurrent, and type-safe
- 📝 **Key-Value Store**: Fast point lookups and prefix scans
- 🔄 **Structured Records**: Schema-free JSON-like memory storage with metadata
- 🔗 **Graph Relationships**: Bidirectional edges with custom relation types
- 🎯 **Vector Embeddings**: Support for high-dimensional embeddings (384/768/1536-dim)
- 📊 **Column Families**: Data isolation with separate column families for KV, records, graph, and vectors
- 🔐 **Optimistic Locking**: Compare-and-swap operations for conflict handling
- 🚦 **Batch Operations**: Efficient bulk inserts and updates
- 📈 **Performance Tuning**: Configurable write buffers, block cache, and compression
- 🎬 **Multimodal File Support**: Built-in types for PDF, DOCX, audio, video, and text processing
- 🤖 **AI/LLM Ready**: Designed for agent memory, document Q&A, and multimodal RAG pipelines
- 📦 **Document Chunking**: Split large files into processable chunks with per-chunk embeddings
- ⚙️ **Customizable Storage**: Configure database location, cache sizes, and vector dimensions
- 🎨 **Colored Console Output**: Beautiful, emoji-rich terminal output for examples and debugging

## Quick Start

### Installation

**Option 1: Using Cargo (Recommended)**

Add OpenDB to your `Cargo.toml`:

```toml
[dependencies]
opendb = "0.1"
```

Or use cargo-add:

```bash
cargo add opendb
```

**Option 2: Pre-built Binaries**

Download platform-specific builds from [GitHub Releases](https://github.com/muhammad-fiaz/opendb/releases):

- **Linux x86_64**: `opendb-linux-x86_64.tar.gz`
- **Linux ARM64**: `opendb-linux-aarch64.tar.gz`
- **macOS Intel**: `opendb-macos-x86_64.tar.gz`
- **macOS Apple Silicon**: `opendb-macos-aarch64.tar.gz`
- **Windows x86_64**: `opendb-windows-x86_64.zip`

See the [Manual Installation](#manual-installation) section below for detailed instructions.

**Option 3: Build from Source**

```bash
git clone https://github.com/muhammad-fiaz/opendb.git
cd opendb
cargo build --release --all-features
```

**Build Requirements:**
- Rust 2021 edition or later
- Clang and LLVM (for RocksDB)
  - Linux: `sudo apt-get install clang llvm`
  - macOS: `brew install llvm`
  - Windows: `choco install llvm`

### Basic Usage

```rust
use opendb::{OpenDB, Memory};
use uuid::Uuid;

// Open database (creates a folder at ./data with multiple files)
let db = OpenDB::open("./data")?;

// Store a memory
let memory = Memory {
    id: Uuid::new_v4().to_string(),
    content: "Hello, OpenDB!".to_string(),
    embedding: vec![0.1, 0.2, 0.3], // 384-dim in production
    importance: 0.8,
    metadata: serde_json::json!({"type": "greeting"}),
    created_at: chrono::Utc::now(),
};

db.insert_memory(&memory)?;

// Retrieve by ID
let retrieved = db.get_memory(&memory.id)?;

// Search by similarity
let query_embedding = vec![0.15, 0.25, 0.35];
let similar = db.search_similar(&query_embedding, 10)?;

// Create relationships
db.link_memories(&memory.id, &other_id, "relates_to")?;
let related = db.get_related(&memory.id)?;
```

### Key-Value Operations

```rust
// Simple KV operations
db.put(b"user:1", b"Alice")?;
let value = db.get(b"user:1")?;

// Prefix scanning
for (key, value) in db.scan_prefix(b"user:")? {
    println!("{:?}: {:?}", key, value);
}
```

### Transactions

```rust
// Begin transaction
let txn = db.begin_transaction()?;

// Transactional operations
txn.put(b"balance", b"1000")?;
txn.put(b"updated_at", current_time.as_bytes())?;

// Commit atomically
db.commit_transaction(txn)?;
```

### Configuration & Custom Storage

OpenDB supports flexible configuration including custom storage locations, cache sizes, and vector dimensions:

```rust
use opendb::{OpenDB, OpenDBOptions};

// Customize all settings with method chaining
let options = OpenDBOptions::new()
    .with_storage_path("./my_custom_db")  // Custom storage location
    .with_kv_cache_size(5000)             // Larger KV cache
    .with_record_cache_size(3000)          // Larger record cache
    .dimension(768);                        // Larger embeddings (e.g., OpenAI)

let db = OpenDB::open_with_options("./data", options)?;
```

**Configuration Options:**
- `with_storage_path()`: Custom database directory (useful for multi-tenant or production deployments)
- `with_kv_cache_size()`: Number of KV entries to cache (default: 1000)
- `with_record_cache_size()`: Number of memory records to cache (default: 500)
- `dimension()`: Embedding vector dimension (default: 384 for sentence-transformers)

**Production Examples:**

```rust
// Environment-based configuration
let db_path = std::env::var("OPENDB_PATH")
    .unwrap_or_else(|_| "./data/prod_db".to_string());

let prod_options = OpenDBOptions::with_dimension(768)
    .with_kv_cache_size(10000)
    .with_record_cache_size(5000);

let db = OpenDB::open_with_options(&db_path, prod_options)?;

// Multi-tenant pattern
for tenant_id in &["tenant_a", "tenant_b", "tenant_c"] {
    let tenant_path = format!("./data/tenants/{}", tenant_id);
    let db = OpenDB::open(&tenant_path)?;
    // Each tenant has isolated database
}
```

See the [custom_storage example](examples/custom_storage.rs) for comprehensive configuration patterns.

Run it with:
```bash
cargo run --example custom_storage
```

### Multimodal AI Applications

OpenDB provides built-in support for multimodal file processing, perfect for AI agents, RAG systems, and document Q&A:

```rust
use opendb::{OpenDB, MultimodalDocument, DocumentChunk, FileType};

let db = OpenDB::open("./ai_agent")?;

// Process a PDF document
let mut pdf_doc = MultimodalDocument::new(
    "research_001",
    "paper.pdf",
    FileType::Pdf,
    1024 * 500, // 500 KB
    "Extracted text from PDF...",
    generate_embedding("paper content"), // Use sentence-transformers
)
.with_metadata("author", "Dr. Smith")
.with_metadata("pages", "15");

// Add chunks for large documents
pdf_doc.add_chunk(DocumentChunk::new(
    "chunk_0",
    "Introduction section...",
    generate_embedding("introduction"),
    0,
    1000,
));

// Supports: PDF, DOCX, TXT, MP3, MP4, WAV, JPG, PNG, and more
let file_type = FileType::from_extension("mp3");
println!("{}", file_type.description()); // "Audio file"
```

See the [multimodal_agent example](examples/multimodal_agent.rs) for a complete demo of:
- PDF, DOCX, and text document processing
- Audio transcription workflows (with whisper-rs)
- Video caption extraction (with ffmpeg)
- Document chunking strategies
- Embedding generation patterns
- Production RAG pipelines

Run it with:
```bash
cargo run --example multimodal_agent
```

## Architecture

OpenDB is built with a modular architecture:

```
┌─────────────────────────────────────────┐
│         OpenDB Core API                 │
├─────────┬──────────┬──────────┬─────────┤
│   KV    │ Records  │  Graph   │ Vector  │
├─────────┴──────────┴──────────┴─────────┤
│         Transaction Layer               │
├─────────────────────────────────────────┤
│         LRU Cache Layer                 │
├─────────────────────────────────────────┤
│      RocksDB Storage Backend            │
└─────────────────────────────────────────┘
```

- **Storage Layer**: RocksDB with column families for data isolation
- **Caching Layer**: Write-through LRU cache with RocksDB block cache
- **Transaction Layer**: Snapshot isolation with optimistic locking
- **API Layer**: Type-safe Rust APIs for all operations

See the [Architecture Documentation](https://muhammad-fiaz.github.io/opendb/architecture/overview.html) for details.

## Database Structure

### OpenDB Database Folder

OpenDB uses a **folder-based architecture** with multiple files for high performance:

```
./my_database/               # Your database folder
├── OPENDB_INFO             # OpenDB metadata (identifies this as OpenDB)
├── README.md               # Database-specific documentation
├── .opendb_config.json     # Database configuration
├── CURRENT                 # Points to current MANIFEST file
├── IDENTITY                # Database UUID
├── LOCK                    # Prevents concurrent access
├── MANIFEST-*              # Database metadata and file list
├── OPTIONS-*               # RocksDB configuration
├── *.log                   # Write-Ahead Log (WAL) for durability
└── *.sst                   # Sorted String Tables (actual data)
```

**Key Files:**

- `OPENDB_INFO`: OpenDB metadata explaining format and features
- `README.md`: Database-specific documentation and backup instructions
- `.opendb_config.json`: Machine-readable database configuration
- `*.log`: Write-Ahead Log ensures durability (changes written here first)
- `*.sst`: Sorted String Table files store the actual data (compressed)
- `MANIFEST`: Tracks which SST files are active
- `LOCK`: Ensures only one process accesses the database at a time

**Benefits of folder-based design:**

- ✅ **Higher write throughput**: WAL allows fast sequential writes
- ✅ **Better compression**: Data is compressed in SST files
- ✅ **Efficient compaction**: Background merging of files
- ✅ **Crash recovery**: WAL enables reliable recovery

**Important notes:**

- ⚠️ Always backup the **entire folder** (not individual files)
- ⚠️ Do NOT manually edit files in the database folder
- ⚠️ Only one process can open a database at a time (enforced by LOCK file)

Check the `OPENDB_INFO`, `README.md`, and `.opendb_config.json` files in any database folder for detailed information.

## Installation

### Prerequisites

- **Rust 1.70+**: [Install Rust](https://rustup.rs/)
- **RocksDB dependencies** (handled automatically by cargo)

### From crates.io

```bash
cargo add opendb
```

### From source

```bash
git clone https://github.com/muhammad-fiaz/OpenDB.git
cd OpenDB
cargo build --release
```

## Examples

OpenDB includes comprehensive examples:

```bash
# Basic quickstart
cargo run --example quickstart

# AI agent memory system with colored output
cargo run --example memory_agent

# Graph relationship traversal
cargo run --example graph_relations

# Multimodal AI/LLM application (PDF, audio, video, text)
cargo run --example multimodal_agent

# Custom storage configuration patterns
cargo run --example custom_storage
```

All examples feature:
- ✨ **Colorful console output** with emojis for better readability
- 📊 **Detailed progress indicators** showing what's happening
- 💡 **Best practices** and production-ready patterns
- 🎯 **Complete workflows** from data ingestion to query

See the [`examples/`](examples/) directory for more.

## Documentation

- 📖 **[User Guide](https://muhammad-fiaz.github.io/opendb)**: Complete documentation
- 📚 **[API Reference](https://muhammad-fiaz.github.io/opendb/api/kv.html)**: Detailed API docs
- 🏗️ **[Architecture](https://muhammad-fiaz.github.io/opendb/architecture/overview.html)**: Design and internals
- ⚡ **[Performance Tuning](https://muhammad-fiaz.github.io/opendb/advanced/performance.html)**: Optimization guide

## Performance

OpenDB delivers excellent performance across all operations:

### Benchmarks

All benchmarks run on a single thread (no parallelism) to show baseline performance:

| Operation | Throughput | Latency (avg) | Description |
|-----------|-----------|---------------|-------------|
| **KV Put** | ~136K ops/sec | 7.36 µs | Write key-value pair to storage |
| **KV Get** | ~10.2M ops/sec | 97.8 ns | Read key-value pair (cached) |
| **Memory Insert** | ~39K ops/sec | 25.5 µs | Insert Memory record with embedding |
| **Memory Get** | ~4.7M ops/sec | 213 ns | Retrieve Memory record by ID |
| **Vector Search (100)** | ~22.7K ops/sec | 44.1 µs | k-NN search across 100 vectors (384-dim) |
| **Vector Search (500)** | ~5.1K ops/sec | 197.6 µs | k-NN search across 500 vectors (384-dim) |
| **Vector Search (1000)** | ~2.5K ops/sec | 400.2 µs | k-NN search across 1000 vectors (384-dim) |
| **Graph Link** | ~54K ops/sec | 18.5 µs | Create bidirectional edge |
| **Graph Get Related** | ~68.3K ops/sec | 14.6 µs | Retrieve outgoing edges |
| **Transaction Commit** | ~129K ops/sec | 7.75 µs | Commit 2-write transaction |

**Notes**:
- Vector search uses brute-force k-NN (linear scan) with euclidean distance
- All operations include full persistence to RocksDB (WAL + LSM writes)
- Benchmarks run with default RocksDB settings
- Memory operations include rkyv serialization/deserialization
- Graph operations maintain bidirectional indices
- ⚠️ **Performance varies by system**: These benchmarks were run on a specific hardware configuration. Your performance may differ based on your OS, platform, processor, RAM, and storage type (SSD vs HDD).

**Reproduce benchmarks on your system:**

Run the benchmarks using the `benches/benchmark.rs` file:

```bash
cargo bench --bench benchmark
```

For detailed results with plots and statistical analysis:

```bash
cargo install cargo-criterion
cargo criterion --bench benchmark
```

The benchmark file (`benches/benchmark.rs`) includes comprehensive tests for:
- Key-value operations (put, get)
- Memory operations (insert, get)
- Vector search (100, 500, 1000 vectors)
- Graph operations (link, get_related)
- Transaction operations (commit)

Run benchmarks yourself:
```bash
cargo bench --bench benchmark
```

### Performance Characteristics

- **KV Operations**: Sub-microsecond reads with LRU cache, single-digit µs writes
- **Vector Search**: Sub-millisecond for datasets up to 1K vectors (384-dim)
- **Graph Traversal**: Constant-time relationship lookups via indices
- **Transactions**: Full ACID with minimal overhead (~7-8 µs commit time)
- **Serialization**: Zero-copy deserialization with rkyv for optimal performance

See [Performance Guide](https://muhammad-fiaz.github.io/opendb/advanced/performance.html) for tuning.

## Manual Installation

Download pre-built binaries from [GitHub Releases](https://github.com/muhammad-fiaz/opendb/releases) for your platform:

### Linux x86_64

```bash
# Download and extract
wget https://github.com/muhammad-fiaz/opendb/releases/latest/download/opendb-linux-x86_64.tar.gz
tar -xzf opendb-linux-x86_64.tar.gz

# System-wide installation (requires sudo)
sudo cp libopendb.so /usr/local/lib/
sudo ldconfig

# Or copy to your project
cp libopendb.* /path/to/your/project/lib/
```

### Linux ARM64 (aarch64)

```bash
wget https://github.com/muhammad-fiaz/opendb/releases/latest/download/opendb-linux-aarch64.tar.gz
tar -xzf opendb-linux-aarch64.tar.gz
sudo cp libopendb.so /usr/local/lib/
sudo ldconfig
```

### macOS x86_64 (Intel)

```bash
curl -L https://github.com/muhammad-fiaz/opendb/releases/latest/download/opendb-macos-x86_64.tar.gz -o opendb-macos-x86_64.tar.gz
tar -xzf opendb-macos-x86_64.tar.gz
sudo cp libopendb.dylib /usr/local/lib/
```

### macOS ARM64 (Apple Silicon)

```bash
curl -L https://github.com/muhammad-fiaz/opendb/releases/latest/download/opendb-macos-aarch64.tar.gz -o opendb-macos-aarch64.tar.gz
tar -xzf opendb-macos-aarch64.tar.gz
sudo cp libopendb.dylib /usr/local/lib/
```

### Windows x86_64

```powershell
# Download and extract
Invoke-WebRequest -Uri "https://github.com/muhammad-fiaz/opendb/releases/latest/download/opendb-windows-x86_64.zip" -OutFile "opendb-windows-x86_64.zip"
Expand-Archive -Path opendb-windows-x86_64.zip -DestinationPath .

# Copy to system PATH or your project directory
Copy-Item opendb.dll C:\Windows\System32\
# Or add to your project directory
```

### Build Dependencies

If building from source or using the library, ensure you have:

- **Clang and LLVM** (required for RocksDB bindings)
  - **Linux**: `sudo apt-get install clang llvm`
  - **macOS**: `brew install llvm`
  - **Windows**: `choco install llvm`

For **Alpine Linux** (musl libc), use the `bindgen-static` feature:

```toml
[dependencies.opendb]
default-features = false
features = ["bindgen-static"]
```

For **Windows /MT runtime**, use the `mt_static` feature:

```toml
[dependencies.opendb]
features = ["mt_static"]
```

## Use Cases

- **AI Agent Memory**: Persistent memory for AI agents with semantic search
- **Knowledge Graphs**: Store entities and relationships with vector embeddings
- **Semantic Search**: Fast similarity search over document embeddings
- **Graph Analytics**: Relationship analysis and traversal
- **Embedded Database**: Lightweight, embedded storage for Rust applications

## Changelog

See the [GitHub Releases](https://github.com/muhammad-fiaz/opendb/releases) page for version history and changelog.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

- 🐛 **Bug Reports**: [Open an issue](https://github.com/muhammad-fiaz/OpenDB/issues/new?template=bug_report.md)
- ✨ **Feature Requests**: [Suggest a feature](https://github.com/muhammad-fiaz/OpenDB/issues/new?template=feature_request.md)
- 🔀 **Pull Requests**: [Submit a PR](https://github.com/muhammad-fiaz/OpenDB/pulls)

## License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed under the Apache License, Version 2.0, without any additional terms or conditions.

## Contact

- **Author**: Muhammad Fiaz
- **Email**: contact@muhammadfiaz.com
- **GitHub**: [@muhammad-fiaz](https://github.com/muhammad-fiaz)
- **Repository**: [muhammad-fiaz/opendb](https://github.com/muhammad-fiaz/opendb)

---

**OpenDB** - High-performance hybrid embedded database for Rust 🦀
