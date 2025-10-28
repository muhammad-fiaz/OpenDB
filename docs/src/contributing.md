# Contributing to OpenDB

Thank you for your interest in contributing to OpenDB! This guide will help you get started.

## Code of Conduct

This project adheres to the Contributor Covenant Code of Conduct. By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

1. **Check existing issues** to avoid duplicates
2. **Use the bug report template** when creating a new issue
3. **Provide details:**
   - OpenDB version
   - Rust version (`rustc --version`)
   - Operating system
   - Minimal reproduction steps
   - Expected vs actual behavior

### Suggesting Features

1. **Check the roadmap** to see if it's planned
2. **Use the feature request template**
3. **Describe:**
   - Use case and motivation
   - Proposed API design
   - Alternative solutions considered

### Pull Requests

1. **Fork** the repository
2. **Create a branch** from `main`:
   ```bash
   git checkout -b feature/my-feature
   ```
3. **Make your changes** following our code style
4. **Write tests** for new functionality
5. **Update documentation** if needed
6. **Commit** with descriptive messages
7. **Push** to your fork
8. **Open a pull request** with detailed description

## Development Setup

### Prerequisites

- Rust 1.70 or later
- RocksDB development libraries (see Installation guide)

### Clone and Build

```bash
git clone https://github.com/muhammad-fiaz/OpenDB.git
cd OpenDB
cargo build
```

### Run Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture
```

### Run Examples

```bash
cargo run --example quickstart
cargo run --example memory_agent
cargo run --example graph_relations
```

### Build Documentation

```bash
# API docs
cargo doc --open

# mdBook docs
cd docs
mdbook serve --open
```

## Code Style

### Formatting

Use `rustfmt` for consistent formatting:

```bash
cargo fmt --all
```

### Linting

Use `clippy` for code quality:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Naming Conventions

- **Types:** `PascalCase` (e.g., `OpenDB`, `StorageBackend`)
- **Functions:** `snake_case` (e.g., `insert_memory`, `get_related`)
- **Constants:** `SCREAMING_SNAKE_CASE` (e.g., `DEFAULT_CACHE_SIZE`)
- **Modules:** `snake_case` (e.g., `graph`, `vector`)

### Documentation

- **Public APIs:** Must have `///` documentation
- **Examples:** Include usage examples in doc comments
- **Errors:** Document possible error cases

Example:

```rust
/// Inserts a memory record into the database.
///
/// # Arguments
///
/// * `memory` - The memory record to insert
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if:
/// - Serialization fails
/// - Storage write fails
///
/// # Example
///
/// ```
/// let memory = Memory::new("id".to_string(), "content".to_string());
/// db.insert_memory(&memory)?;
/// ```
pub fn insert_memory(&self, memory: &Memory) -> Result<()> {
    // ...
}
```

## Testing Guidelines

### Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_creation() {
        let memory = Memory::new("id".to_string(), "content".to_string());
        assert_eq!(memory.id, "id");
        assert_eq!(memory.content, "content");
    }
}
```

### Integration Tests

Place integration tests in `tests/`:

```rust
// tests/my_feature_test.rs
use opendb::{OpenDB, Memory};
use tempfile::TempDir;

#[test]
fn test_my_feature() {
    let temp_dir = TempDir::new().unwrap();
    let db = OpenDB::open(temp_dir.path()).unwrap();
    
    // Test logic
}
```

### Test Coverage

Aim for:

- **New features:** >80% coverage
- **Bug fixes:** Regression test included
- **Edge cases:** Test error paths

## Commit Messages

Follow conventional commits format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Formatting changes
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

**Examples:**

```
feat(graph): add weighted edge support

Adds optional weight parameter to link() method,
allowing users to specify edge weights.

Closes #123
```

```
fix(cache): prevent race condition in LRU eviction

Fixes deadlock when multiple threads evict simultaneously
by using a write lock during eviction.

Fixes #456
```

## Pull Request Guidelines

### PR Title

Use the same format as commit messages:

```
feat(vector): add cosine similarity distance metric
```

### PR Description

Include:

1. **What:** Description of changes
2. **Why:** Motivation and context
3. **How:** Implementation approach
4. **Testing:** How you tested the changes
5. **Checklist:**
   - [ ] Tests added/updated
   - [ ] Documentation updated
   - [ ] Changelog updated (for features/fixes)
   - [ ] Code formatted with `rustfmt`
   - [ ] Linted with `clippy`

### Review Process

1. **CI checks:** All tests must pass
2. **Code review:** At least one maintainer approval
3. **Documentation:** Verify docs are updated
4. **Changelog:** Ensure CHANGELOG.md is updated

## Architecture Guidelines

### Module Organization

Follow existing structure:

```
src/
  lib.rs          # Public API exports
  database.rs     # Main OpenDB struct
  error.rs        # Error types
  types.rs        # Core data types
  storage/        # Storage backends
  cache/          # Caching layer
  kv/             # Key-value store
  records/        # Memory records
  graph/          # Graph relationships
  vector/         # Vector search
  transaction/    # Transaction management
  codec/          # Serialization
```

### Adding New Features

1. **New module:** Create in appropriate directory
2. **Trait-based:** Use traits for extensibility
3. **Error handling:** Use `Result<T, Error>`
4. **Thread safety:** Ensure `Send + Sync` where needed

## Performance Considerations

- **Benchmarks:** Add benchmarks for performance-critical code
- **Profiling:** Profile before optimizing
- **Allocations:** Minimize unnecessary allocations
- **Locks:** Prefer `RwLock` for read-heavy workloads

## Documentation Updates

When adding features, update:

1. **API docs:** `///` comments in code
2. **mdBook docs:** Relevant pages in `docs/src/`
3. **Examples:** Add example if appropriate
4. **CHANGELOG.md:** Document changes
5. **README.md:** Update if API changes

## Release Process (Maintainers)

1. **Version bump:** Update `Cargo.toml`
2. **Changelog:** Update `CHANGELOG.md`
3. **Tag:** Create git tag `v0.x.y`
4. **Publish:** `cargo publish`
5. **GitHub Release:** Create release notes

## Getting Help

- **Discussions:** GitHub Discussions for questions
- **Issues:** GitHub Issues for bugs/features
- **Email:** contact@muhammadfiaz.com for private inquiries

## Recognition

Contributors are recognized in:

- `CONTRIBUTORS.md` file
- GitHub contributors page
- Release notes

Thank you for contributing to OpenDB! 🎉

## Next

- [Roadmap](roadmap.md)
- [Architecture](../architecture/overview.md)
