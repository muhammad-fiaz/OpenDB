# Installation

## From crates.io (once published)

```bash
cargo add opendb
```

## From source

1. Clone the repository:

```bash
git clone https://github.com/muhammad-fiaz/OpenDB.git
cd OpenDB
```

2. Build the project:

```bash
cargo build --release
```

3. Run tests:

```bash
cargo test
```

4. Run examples:

```bash
cargo run --example quickstart
cargo run --example memory_agent
cargo run --example graph_relations
```

## Requirements

- **Rust**: 1.70.0 or higher (Rust 2021 edition)
- **Operating System**: Linux, macOS, or Windows
- **Dependencies**: All dependencies are managed by Cargo

## System Dependencies

OpenDB uses RocksDB as its storage backend, which requires:

- **Linux**: gcc, g++, make, libsnappy-dev, zlib1g-dev, libbz2-dev, liblz4-dev
- **macOS**: Xcode command line tools
- **Windows**: Visual Studio Build Tools

### Linux Setup

```bash
# Ubuntu/Debian
sudo apt-get install -y gcc g++ make libsnappy-dev zlib1g-dev libbz2-dev liblz4-dev

# Fedora/RHEL
sudo dnf install -y gcc gcc-c++ make snappy-devel zlib-devel bzip2-devel lz4-devel
```

### macOS Setup

```bash
xcode-select --install
```

### Windows Setup

Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)

## Verifying Installation

```bash
cargo test --all
```

All tests should pass. If you encounter issues, please check:

1. Rust version: `rustc --version`
2. Build dependencies are installed
3. [Open an issue](https://github.com/muhammad-fiaz/OpenDB/issues) if problems persist
