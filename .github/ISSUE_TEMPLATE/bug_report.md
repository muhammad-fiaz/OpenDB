---
name: Bug report
about: Create a report to help us improve OpenDB
title: '[BUG] '
labels: bug
assignees: muhammad-fiaz

---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Initialize database with '...'
2. Run operation '...'
3. See error

**Code Sample**
```rust
// Minimal code to reproduce the issue
let db = OpenDB::open("./data")?;
// ...
```

**Expected behavior**
A clear and concise description of what you expected to happen.

**Actual behavior**
What actually happened, including error messages.

**Error Output**
```
Paste error messages or stack traces here
```

**Environment:**
 - OS: [e.g., Ubuntu 22.04, Windows 11, macOS 14]
 - Rust version: [e.g., 1.70.0]
 - OpenDB version: [e.g., 0.1.0]
 - RocksDB version: [e.g., 0.24.0]

**Additional context**
Add any other context about the problem here, such as:
- Database size
- Number of records
- Concurrent operations
- Configuration settings
