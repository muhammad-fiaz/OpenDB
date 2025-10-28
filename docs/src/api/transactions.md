# Transactions API

OpenDB provides ACID-compliant transactions for atomic multi-operation updates.

## Overview

Transactions group multiple operations into a single atomic unit:

```rust
use opendb::OpenDB;

let db = OpenDB::open("./db")?;
let mut txn = db.begin_transaction()?;

txn.put("default", b"key1", b"value1")?;
txn.put("default", b"key2", b"value2")?;
txn.commit()?; // Both writes succeed or both fail
```

## Basic API

### Begin Transaction

```rust
let mut txn = db.begin_transaction()?;
```

**Signature:**

```rust
pub fn begin_transaction(&self) -> Result<Transaction>
```

**Returns:** `Transaction` handle for performing operations.

### Commit

```rust
txn.commit()?;
```

**Signature:**

```rust
pub fn commit(mut self) -> Result<()>
```

**Behavior:**

- Atomically applies all changes
- Returns error if conflicts detected (optimistic locking)
- Consumes transaction (can't use after commit)

### Rollback

```rust
txn.rollback()?;
```

**Signature:**

```rust
pub fn rollback(mut self) -> Result<()>
```

**Behavior:**

- Discards all changes
- Always succeeds
- Consumes transaction

### Auto-Rollback

Transactions auto-rollback if dropped without commit:

```rust
{
    let mut txn = db.begin_transaction()?;
    txn.put("default", b"key", b"value")?;
    // txn dropped here → automatic rollback
}

// Key was not written
assert!(db.get(b"key")?.is_none());
```

## Transaction Operations

### Get

```rust
let value = txn.get("default", b"key")?;
```

**Signature:**

```rust
pub fn get(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>>
```

**Behavior:**

- Reads from transaction snapshot
- Sees writes from current transaction
- Isolated from concurrent transactions

### Put

```rust
txn.put("default", b"key", b"value")?;
```

**Signature:**

```rust
pub fn put(&mut self, cf: &str, key: &[u8], value: &[u8]) -> Result<()>
```

**Behavior:**

- Buffers write in transaction
- Not visible outside transaction until commit
- Visible to subsequent reads in same transaction

### Delete

```rust
txn.delete("default", b"key")?;
```

**Signature:**

```rust
pub fn delete(&mut self, cf: &str, key: &[u8]) -> Result<()>
```

**Behavior:**

- Buffers delete in transaction
- Subsequent gets in same transaction return `None`

## Column Families

Transactions work across all column families:

```rust
let mut txn = db.begin_transaction()?;

// Write to different column families
txn.put("default", b"kv_key", b"value")?;
txn.put("records", b"mem_001", &encoded_memory)?;
txn.put("graph_forward", b"mem_001:related_to", &edges)?;

txn.commit()?; // All or nothing
```

**Available Column Families:**

- `"default"` - KV store
- `"records"` - Memory records
- `"graph_forward"` - Outgoing edges
- `"graph_backward"` - Incoming edges
- `"vector_data"` - Embedding data
- `"vector_index"` - HNSW index
- `"metadata"` - Database metadata

## ACID Examples

### Atomicity

Either all operations succeed or none:

```rust
let mut txn = db.begin_transaction()?;

txn.put("default", b"account_A", b"-100")?;
txn.put("default", b"account_B", b"+100")?;

match txn.commit() {
    Ok(_) => println!("Transfer complete"),
    Err(e) => println!("Transfer failed, both accounts unchanged: {}", e),
}
```

### Consistency

Maintain invariants across operations:

```rust
// Invariant: memory must exist before linking
let mut txn = db.begin_transaction()?;

// Insert memories
txn.put("records", b"mem_001", &encode_memory(&mem1))?;
txn.put("records", b"mem_002", &encode_memory(&mem2))?;

// Create link (requires both memories exist)
txn.put("graph_forward", b"mem_001:related_to", &encode_edges(&edges))?;

txn.commit()?; // Ensures consistency
```

### Isolation

Transactions don't see each other's uncommitted changes:

```rust
// Transaction 1
let mut txn1 = db.begin_transaction()?;
txn1.put("default", b"counter", b"100")?;

// Transaction 2 (concurrent)
let mut txn2 = db.begin_transaction()?;
let val = txn2.get("default", b"counter")?; // Sees old value (not 100)

txn1.commit()?;
txn2.commit()?; // May conflict depending on operations
```

### Durability

Committed changes survive crashes:

```rust
let mut txn = db.begin_transaction()?;
txn.put("default", b"important", b"data")?;
txn.commit()?;

// Even if process crashes here, data is safe

// Reopen database
let db = OpenDB::open("./db")?;
assert_eq!(db.get(b"important")?.unwrap(), b"data");
```

## Conflict Handling

Transactions use optimistic locking and may fail on conflict:

```rust
use opendb::Error;

loop {
    let mut txn = db.begin_transaction()?;
    
    // Read-modify-write
    let val = txn.get("default", b"counter")?
        .and_then(|v| String::from_utf8(v).ok())
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    
    let new_val = val + 1;
    txn.put("default", b"counter", new_val.to_string().as_bytes())?;
    
    match txn.commit() {
        Ok(_) => break,
        Err(Error::Transaction(_)) => {
            println!("Conflict detected, retrying...");
            continue; // Retry
        }
        Err(e) => return Err(e),
    }
}
```

## Advanced Patterns

### Compare-and-Swap

```rust
fn compare_and_swap(
    db: &OpenDB,
    key: &[u8],
    expected: &[u8],
    new_value: &[u8],
) -> Result<bool> {
    let mut txn = db.begin_transaction()?;
    
    let current = txn.get("default", key)?;
    if current.as_deref() != Some(expected) {
        txn.rollback()?;
        return Ok(false); // Value changed
    }
    
    txn.put("default", key, new_value)?;
    txn.commit()?;
    Ok(true)
}
```

### Batch Updates

```rust
fn batch_update(db: &OpenDB, updates: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()> {
    let mut txn = db.begin_transaction()?;
    
    for (key, value) in updates {
        txn.put("default", &key, &value)?;
    }
    
    txn.commit()
}
```

### Conditional Delete

```rust
fn delete_if_exists(db: &OpenDB, key: &[u8]) -> Result<bool> {
    let mut txn = db.begin_transaction()?;
    
    if txn.get("default", key)?.is_none() {
        txn.rollback()?;
        return Ok(false);
    }
    
    txn.delete("default", key)?;
    txn.commit()?;
    Ok(true)
}
```

## Performance Considerations

### Transaction Overhead

Transactions have overhead compared to direct writes:

```rust
// ❌ Slower: Many small transactions
for i in 0..1000 {
    let mut txn = db.begin_transaction()?;
    txn.put("default", &format!("key_{}", i).as_bytes(), b"value")?;
    txn.commit()?;
}

// ✅ Faster: One transaction for batch
let mut txn = db.begin_transaction()?;
for i in 0..1000 {
    txn.put("default", &format!("key_{}", i).as_bytes(), b"value")?;
}
txn.commit()?;
```

### Transaction Size

Keep transactions reasonably sized:

- **Small (1-100 ops):** Best performance
- **Medium (100-1000 ops):** Good
- **Large (1000+ ops):** May increase conflict rate and memory usage

### Conflict Rate

High contention increases conflict rate:

```rust
// High contention: many threads updating same key
// Solution: Shard keys or use separate counters
```

## Limitations

1. **Single-threaded:** One transaction per thread
2. **No nested transactions:** Can't begin transaction within transaction
3. **Memory buffering:** Large transactions use more memory
4. **Optimistic locking:** High contention may cause retries

## Error Handling

```rust
use opendb::Error;

let mut txn = db.begin_transaction()?;
txn.put("default", b"key", b"value")?;

match txn.commit() {
    Ok(_) => println!("Success"),
    Err(Error::Transaction(e)) => println!("Conflict: {}", e),
    Err(Error::Storage(e)) => println!("Storage error: {}", e),
    Err(e) => println!("Other error: {}", e),
}
```

## Best Practices

1. **Keep transactions short:** Minimize duration to reduce conflicts
2. **Handle conflicts:** Implement retry logic for read-modify-write
3. **Batch when possible:** Group related operations
4. **Use auto-rollback:** Let Drop handle cleanup in error paths
5. **Explicit commits:** Don't rely on implicit behavior

## Next

- [Architecture: Transactions](../architecture/transactions.md)
- [Performance Tuning](../advanced/performance.md)
