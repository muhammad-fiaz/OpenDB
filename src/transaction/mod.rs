// Transaction management for ACID guarantees
//
// This module provides transaction support with full ACID semantics.

pub mod manager;

use crate::error::Result;
use crate::storage::Transaction as StorageTransaction;

/// Transaction handle for ACID operations
pub struct Transaction {
    inner: Option<Box<dyn StorageTransaction>>,
    active: bool,
}

impl Transaction {
    /// Create a new transaction from a storage transaction
    pub(crate) fn new(txn: Box<dyn StorageTransaction>) -> Self {
        Self {
            inner: Some(txn),
            active: true,
        }
    }

    /// Get a value within this transaction
    pub fn get(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>> {
        if !self.active {
            return Err(crate::error::Error::Transaction("Transaction not active".to_string()));
        }
        self.inner.as_ref()
            .ok_or_else(|| crate::error::Error::Transaction("Transaction not active".to_string()))?
            .get(cf, key)
    }

    /// Put a key-value pair within this transaction
    pub fn put(&mut self, cf: &str, key: &[u8], value: &[u8]) -> Result<()> {
        if !self.active {
            return Err(crate::error::Error::Transaction("Transaction not active".to_string()));
        }
        self.inner.as_mut()
            .ok_or_else(|| crate::error::Error::Transaction("Transaction not active".to_string()))?
            .put(cf, key, value)
    }

    /// Delete a key within this transaction
    pub fn delete(&mut self, cf: &str, key: &[u8]) -> Result<()> {
        if !self.active {
            return Err(crate::error::Error::Transaction("Transaction not active".to_string()));
        }
        self.inner.as_mut()
            .ok_or_else(|| crate::error::Error::Transaction("Transaction not active".to_string()))?
            .delete(cf, key)
    }

    /// Commit the transaction
    pub fn commit(mut self) -> Result<()> {
        if !self.active {
            return Err(crate::error::Error::Transaction("Transaction already completed".to_string()));
        }
        self.active = false;
        self.inner.take()
            .ok_or_else(|| crate::error::Error::Transaction("Transaction not active".to_string()))?
            .commit()
    }

    /// Rollback the transaction
    pub fn rollback(mut self) -> Result<()> {
        if !self.active {
            return Err(crate::error::Error::Transaction("Transaction already completed".to_string()));
        }
        self.active = false;
        self.inner.take()
            .ok_or_else(|| crate::error::Error::Transaction("Transaction not active".to_string()))?
            .rollback()
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        // Auto-rollback if not committed
        if self.active {
            // Consume self.inner without calling methods
            // The underlying transaction will handle cleanup
        }
    }
}
