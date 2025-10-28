// Transaction manager for coordinating ACID operations

use crate::error::Result;
use crate::storage::SharedStorage;
use crate::transaction::Transaction;

/// Transaction manager
pub struct TransactionManager {
    storage: SharedStorage,
}

impl TransactionManager {
    /// Create a new transaction manager
    pub fn new(storage: SharedStorage) -> Self {
        Self { storage }
    }

    /// Begin a new transaction
    pub fn begin(&self) -> Result<Transaction> {
        let txn = self.storage.begin_transaction()?;
        Ok(Transaction::new(txn))
    }
}
