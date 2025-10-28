# Key-Value Store API

OpenDB provides a simple, fast key-value interface for storing arbitrary binary data.

## Basic Operations

### Put

Store a value under a key:

```rust
use opendb::OpenDB;

let db = OpenDB::open("./db")?;
db.put(b"user:123", b"Alice")?;
```

**Signature:**

```rust
pub fn put(&self, key: &[u8], value: &[u8]) -> Result<()>
```

**Behavior:**

- Writes to storage immediately (write-through cache)
- Updates cache
- Returns error if storage fails

### Get

Retrieve a value by key:

```rust
let value = db.get(b"user:123")?;
match value {
    Some(bytes) => println!("Found: {}", String::from_utf8_lossy(&bytes)),
    None => println!("Not found"),
}
```

**Signature:**

```rust
pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>
```

**Behavior:**

- Checks cache first (fast path)
- Falls back to storage on cache miss
- Returns `None` if key doesn't exist

### Delete

Remove a key-value pair:

```rust
db.delete(b"user:123")?;
```

**Signature:**

```rust
pub fn delete(&self, key: &[u8]) -> Result<()>
```

**Behavior:**

- Removes from storage
- Invalidates cache entry
- Succeeds even if key doesn't exist

### Exists

Check if a key exists without fetching the value:

```rust
if db.exists(b"user:123")? {
    println!("User exists");
}
```

**Signature:**

```rust
pub fn exists(&self, key: &[u8]) -> Result<bool>
```

**Behavior:**

- Checks cache first
- Falls back to storage on cache miss
- More efficient than `get()` for existence checks

## Advanced Operations

### Scan Prefix

Iterate over all keys with a common prefix:

```rust
let users = db.scan_prefix(b"user:")?;
for (key, value) in users {
    println!("{} = {}", 
        String::from_utf8_lossy(&key),
        String::from_utf8_lossy(&value)
    );
}
```

**Signature:**

```rust
pub fn scan_prefix(&self, prefix: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>>
```

**Behavior:**

- Bypasses cache (reads from storage)
- Returns all matching key-value pairs
- Sorted by key (lexicographic order)

## Usage Patterns

### Namespacing

Use prefixes to organize data:

```rust
// User namespace
db.put(b"user:123", b"Alice")?;
db.put(b"user:456", b"Bob")?;

// Session namespace
db.put(b"session:abc", b"user:123")?;
db.put(b"session:xyz", b"user:456")?;

// Scan all users
let users = db.scan_prefix(b"user:")?;
```

### Counter

Implement atomic counters with transactions:

```rust
fn increment_counter(db: &OpenDB, key: &[u8]) -> Result<u64> {
    let mut txn = db.begin_transaction()?;
    
    let current = txn.get("default", key)?
        .map(|v| u64::from_le_bytes(v.try_into().unwrap()))
        .unwrap_or(0);
    
    let new_val = current + 1;
    txn.put("default", key, &new_val.to_le_bytes())?;
    txn.commit()?;
    
    Ok(new_val)
}

let count = increment_counter(&db, b"visits")?;
```

### Binary Data

Store any serializable type:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,
}

let config = Config {
    host: "localhost".to_string(),
    port: 8080,
};

// Serialize
let bytes = bincode::serialize(&config)?;
db.put(b"config", &bytes)?;

// Deserialize
let bytes = db.get(b"config")?.unwrap();
let config: Config = bincode::deserialize(&bytes)?;
```

## Performance Characteristics

| Operation | Time Complexity | Cache Hit | Cache Miss |
|-----------|----------------|-----------|------------|
| `get()`   | O(1) avg       | ~100ns    | ~1-10µs    |
| `put()`   | O(log n)       | ~1-10µs   | ~1-10µs    |
| `delete()` | O(log n)      | ~1-10µs   | ~1-10µs    |
| `exists()` | O(1) avg      | ~100ns    | ~1-10µs    |
| `scan_prefix()` | O(k log n) | N/A     | ~10µs + k*1µs |

Where:
- `n` = total keys in database
- `k` = number of matching keys

## Error Handling

All operations return `Result<T, Error>`:

```rust
use opendb::{OpenDB, Error};

match db.get(b"key") {
    Ok(Some(value)) => { /* use value */ },
    Ok(None) => { /* key not found */ },
    Err(Error::Storage(e)) => { /* storage error */ },
    Err(Error::Cache(e)) => { /* cache error */ },
    Err(e) => { /* other error */ },
}
```

## Thread Safety

All KV operations are thread-safe:

```rust
use std::sync::Arc;
use std::thread;

let db = Arc::new(OpenDB::open("./db")?);

let handles: Vec<_> = (0..10).map(|i| {
    let db = Arc::clone(&db);
    thread::spawn(move || {
        db.put(format!("key_{}", i).as_bytes(), b"value").unwrap();
    })
}).collect();

for handle in handles {
    handle.join().unwrap();
}
```

## Next

- [Records API](records.md)
- [Transactions](transactions.md)
