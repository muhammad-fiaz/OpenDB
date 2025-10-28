# Contributing to OpenDB

Thank you for your interest in contributing to OpenDB! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to the Contributor Covenant [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to contact@muhammadfiaz.com.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check existing issues. When creating a bug report, include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide specific examples** to demonstrate the steps
- **Describe the behavior you observed** and what you expected
- **Include code samples** and error messages
- **Specify your environment**: OS, Rust version, OpenDB version

Use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.md).

### Suggesting Features

Feature requests are tracked as GitHub issues. When creating a feature request:

- **Use a clear and descriptive title**
- **Provide a detailed description** of the proposed feature
- **Explain why this feature would be useful** to OpenDB users
- **Provide examples** of how the feature would be used
- **Consider implementation details** if applicable

Use the [feature request template](.github/ISSUE_TEMPLATE/feature_request.md).

### Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Make your changes** following the code style guidelines
3. **Add tests** for your changes
4. **Update documentation** if needed
5. **Ensure all tests pass**: `cargo test`
6. **Run formatting**: `cargo fmt`
7. **Run linting**: `cargo clippy`
8. **Commit with clear messages** following the commit message guidelines
9. **Open a pull request** with a clear description

## Development Setup

### Prerequisites

- **Rust 1.70+**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: For version control
- **mdBook** (optional): For documentation (`cargo install mdbook`)

### Clone and Build

```bash
git clone https://github.com/muhammad-fiaz/OpenDB.git
cd OpenDB
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration_test

# Run doctests
cargo test --doc
```

### Running Examples

```bash
cargo run --example quickstart
cargo run --example memory_agent
cargo run --example graph_relations
```

### Building Documentation

```bash
# Build and serve mdBook docs
cd docs
mdbook serve

# Generate API docs
cargo doc --open
```

## Code Style Guidelines

### Formatting

OpenDB uses **rustfmt** for consistent formatting:

```bash
cargo fmt
```

Configuration is in `rustfmt.toml`. Always run before committing.

### Linting

Use **clippy** for linting:

```bash
cargo clippy -- -D warnings
```

All clippy warnings must be resolved before merging.

### Naming Conventions

- **Types**: `PascalCase` (e.g., `OpenDB`, `VectorManager`)
- **Functions/Methods**: `snake_case` (e.g., `insert_memory`, `get_related`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `DEFAULT_CACHE_SIZE`)
- **Modules**: `snake_case` (e.g., `vector`, `graph`)

### Documentation

- **Public APIs**: Must have doc comments (`///`)
- **Modules**: Should have module-level docs (`//!`)
- **Examples**: Include examples in doc comments when helpful
- **Panics**: Document when functions can panic
- **Errors**: Document error conditions
- **Safety**: Document unsafe code requirements

Example:

```rust
/// Inserts a memory into the database.
///
/// # Arguments
///
/// * `memory` - The memory to insert
///
/// # Errors
///
/// Returns `Error::InvalidVector` if the embedding dimension is incorrect.
///
/// # Examples
///
/// ```
/// let memory = Memory { ... };
/// db.insert_memory(&memory)?;
/// ```
pub fn insert_memory(&self, memory: &Memory) -> Result<()> {
    // ...
}
```

## Testing Guidelines

### Unit Tests

- Place tests in the same file as the code (in a `#[cfg(test)]` module)
- Test both success and failure cases
- Use descriptive test names: `test_insert_memory_success`, `test_invalid_dimension_error`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_memory_success() {
        // ...
    }

    #[test]
    fn test_invalid_dimension_error() {
        // ...
    }
}
```

### Integration Tests

- Place in `tests/` directory
- Test cross-module functionality
- Test real-world usage scenarios

### Test Coverage

- Aim for high coverage of public APIs
- Include edge cases and error paths
- Test concurrent scenarios when applicable

## Commit Message Guidelines

Use clear, descriptive commit messages following this format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation changes
- **style**: Formatting, missing semicolons, etc.
- **refactor**: Code restructuring without behavior changes
- **perf**: Performance improvements
- **test**: Adding or updating tests
- **chore**: Maintenance tasks

### Examples

```
feat(vector): add cosine similarity distance metric

Implement cosine similarity as an alternative to Euclidean distance
for vector search. Add configuration option to choose metric.

Closes #123
```

```
fix(cache): prevent cache invalidation race condition

Use write lock when invalidating cache to prevent concurrent
modifications during index rebuild.

Fixes #456
```

## Architecture Guidelines

### Modularity

- Keep modules focused and cohesive
- Use clear interfaces between modules
- Avoid circular dependencies

### Error Handling

- Use `Result<T>` for fallible operations
- Define errors in `error.rs` using `thiserror`
- Provide context in error messages

### Performance

- Avoid unnecessary allocations
- Use zero-copy serialization where possible
- Benchmark performance-critical code
- Profile before optimizing

### Concurrency

- Use `Arc` for shared ownership
- Use `RwLock` or `Mutex` for interior mutability
- Document thread-safety guarantees
- Avoid deadlocks with consistent lock ordering

## Benchmarking

Add benchmarks for performance-critical code:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_insert(c: &mut Criterion) {
    c.bench_function("insert memory", |b| {
        b.iter(|| {
            // benchmark code
        });
    });
}

criterion_group!(benches, bench_insert);
criterion_main!(benches);
```

Run benchmarks:

```bash
cargo bench
```

## Questions?

- Open a [discussion](https://github.com/muhammad-fiaz/OpenDB/discussions)
- Email: contact@muhammadfiaz.com

Thank you for contributing to OpenDB! 🎉
