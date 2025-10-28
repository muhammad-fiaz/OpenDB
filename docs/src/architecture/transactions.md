# Transaction Model

OpenDB provides full ACID (Atomicity, Consistency, Isolation, Durability) guarantees through RocksDB's transaction support.

## ACID Properties

### Atomicity

All operations in a transaction either succeed together or fail together.

```rust
let mut txn = db.begin_transaction()?;
txn.put("records", b"key1", b"value1")?;
txn.put("records", b"key2", b"value2")?;
txn.commit()?; // Both writes succeed or both fail
```

### Consistency

Transactions move the database from one consistent state to another.

### Isolation

Transactions use **snapshot isolation**:

- Each transaction sees a consistent snapshot of the database
- Concurrent transactions don't interfere with each other
- RocksDB provides MVCC (Multi-Version Concurrency Control)

### Durability

Once a transaction commits, the changes are permanent:

- Write-Ahead Log (WAL) ensures durability
- Data survives process crashes
- Can be verified by reopening the database

## Transaction API

### Basic Usage

```rust
// Begin transaction
let mut txn = db.begin_transaction()?;

// Perform operations
txn.put("records", b"key1", b"value1")?;
let val = txn.get("records", b"key1")?;

// Commit
txn.commit()?;
```

### Rollback

```rust
let mut txn = db.begin_transaction()?;
txn.put("records", b"key1", b"modified")?;

// Something went wrong, rollback
txn.rollback()?;

// Original value remains unchanged
```

### Auto-Rollback

Transactions are automatically rolled back if dropped without commit:

```rust
{
    let mut txn = db.begin_transaction()?;
    txn.put("records", b"key1", b"value")?;
    // txn dropped here - auto rollback
}
```

## Concurrency Model

### Optimistic Locking

RocksDB transactions use optimistic locking:

1. Read phase: Transaction reads data without locks
2. Validation phase: Before commit, check if data changed
3. Write phase: If no conflicts, commit; otherwise abort

### Conflict Detection

```rust
// Transaction 1
let mut txn1 = db.begin_transaction()?;
txn1.put("records", b"counter", b"1")?;

// Transaction 2 (concurrent)
let mut txn2 = db.begin_transaction()?;
txn2.put("records", b"counter", b"2")?;

// First to commit wins
txn1.commit()?; // Success
txn2.commit()?; // May fail with conflict error
```

## Snapshot Isolation Example

```rust
// Initial state: counter = 0
db.put(b"counter", b"0")?;

// Transaction 1 reads
let mut txn1 = db.begin_transaction()?;
let val1 = txn1.get("default", b"counter")?;

// Meanwhile, Transaction 2 updates
let mut txn2 = db.begin_transaction()?;
txn2.put("default", b"counter", b"5")?;
txn2.commit()?;

// Transaction 1 still sees old snapshot
let val1_again = txn1.get("default", b"counter")?;
assert_eq!(val1, val1_again); // Still "0"
```

## Best Practices

### Keep Transactions Short

```rust
// ❌ Bad: Long-running transaction
let mut txn = db.begin_transaction()?;
for i in 0..1_000_000 {
    txn.put("default", &i.to_string().as_bytes(), b"value")?;
}
txn.commit()?;

// ✅ Good: Batch commits
for chunk in (0..1_000_000).collect::<Vec<_>>().chunks(1000) {
    let mut txn = db.begin_transaction()?;
    for i in chunk {
        txn.put("default", &i.to_string().as_bytes(), b"value")?;
    }
    txn.commit()?;
}
```

### Handle Conflicts

```rust
loop {
    let mut txn = db.begin_transaction()?;
    
    // Read-modify-write
    let val = txn.get("default", b"counter")?.unwrap_or_default();
    let new_val = increment(val);
    txn.put("default", b"counter", &new_val)?;
    
    match txn.commit() {
        Ok(_) => break,
        Err(Error::Transaction(_)) => continue, // Retry on conflict
        Err(e) => return Err(e),
    }
}
```

### Use Snapshots for Consistent Reads

For read-only operations across multiple keys, use snapshots (coming soon):

```rust
let snapshot = db.snapshot()?;
let val1 = snapshot.get("records", b"key1")?;
let val2 = snapshot.get("records", b"key2")?;
// val1 and val2 are from the same consistent point in time
```

## Limitations

- Transactions are single-threaded (one transaction per thread)
- Cross-column-family transactions are supported
- Very large transactions may impact performance

## Next

- [Caching Strategy](caching.md)
- [API Reference](../api/transactions.md)
