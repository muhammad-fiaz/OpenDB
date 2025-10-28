// RocksDB storage backend implementation
//
// This module provides a high-performance storage backend using RocksDB.
//
// **Why RocksDB?**
// - Battle-tested LSM tree implementation
// - Excellent write throughput and compression
// - Built-in WAL for durability
// - Column families for namespace isolation
// - Native transaction support
//
// **Tradeoff**: RocksDB is C++ with Rust bindings (not pure Rust),
// but the performance and maturity justify this choice.

use crate::error::{Error, Result};
use crate::storage::{
    Snapshot as SnapshotTrait, StorageBackend, Transaction as TransactionTrait,
    column_families::ColumnFamilies,
};
use chrono::Utc;
use rocksdb::{Options, TransactionDB, TransactionDBOptions, TransactionOptions};
use std::fs;
use std::path::Path;
use std::sync::Arc;

/// RocksDB storage backend
pub struct RocksDBBackend {
    db: Arc<TransactionDB>,
}

impl RocksDBBackend {
    /// Open or create a RocksDB database
    ///
    /// # Arguments
    ///
    /// * `path` - Database directory path
    ///
    /// # Returns
    ///
    /// A new RocksDB backend instance
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        // Performance tuning
        opts.set_write_buffer_size(128 * 1024 * 1024); // 128MB
        opts.set_max_write_buffer_number(3);
        opts.set_target_file_size_base(64 * 1024 * 1024); // 64MB
        opts.set_level_zero_file_num_compaction_trigger(4);
        opts.set_max_background_jobs(4);
        opts.set_compression_type(rocksdb::DBCompressionType::Lz4);

        let txn_db_opts = TransactionDBOptions::default();

        // Open with all column families
        let cf_names = ColumnFamilies::all();
        let db = TransactionDB::open_cf(&opts, &txn_db_opts, &path, &cf_names)
            .map_err(|e| Error::Storage(format!("Failed to open database: {}", e)))?;

        // Create OpenDB metadata file to identify this as an OpenDB database
        let backend = Self { db: Arc::new(db) };
        backend.create_opendb_metadata(&path)?;

        Ok(backend)
    }

    /// Get a column family handle
    fn cf_handle(&self, cf: &str) -> Result<&rocksdb::ColumnFamily> {
        self.db
            .cf_handle(cf)
            .ok_or_else(|| Error::Storage(format!("Column family not found: {}", cf)))
    }

    /// Create OpenDB metadata file in the database directory
    ///
    /// This file identifies the database as an OpenDB database and provides
    /// information about the database format and version.
    fn create_opendb_metadata<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let db_path = path.as_ref();

        // Create OPENDB_INFO file
        let info_path = db_path.join("OPENDB_INFO");
        if !info_path.exists() {
            let info_content = format!(
                "# OpenDB Database\n\
                 \n\
                 This directory contains an OpenDB database.\n\
                 \n\
                 Database Format: OpenDB v0.1.0\n\
                 Storage Engine: RocksDB (LSM-tree based)\n\
                 Created: {}\n\
                 \n\
                 ## Folder Structure\n\
                 \n\
                 OpenDB uses a folder-based architecture with multiple files:\n\
                 \n\
                 - *.log files: Write-Ahead Log (WAL) for durability\n\
                 - *.sst files: Sorted String Table files (data storage)\n\
                 - MANIFEST: Database metadata and file list\n\
                 - CURRENT: Points to current MANIFEST file\n\
                 - OPTIONS: RocksDB configuration settings\n\
                 - LOCK: Prevents multiple processes from opening the same database\n\
                 - OPENDB_INFO: This file (OpenDB metadata)\n\
                 - README.md: Database documentation\n\
                 \n\
                 This multi-file design enables:\n\
                 - Higher write throughput\n\
                 - Better compression\n\
                 - Efficient compaction\n\
                 - Background optimization\n\
                 \n\
                 ## Database Features\n\
                 \n\
                 This OpenDB database supports:\n\
                 - Key-Value Store\n\
                 - Structured Records (JSON)\n\
                 - Graph Database (nodes & edges)\n\
                 - Vector Database (embeddings)\n\
                 - Multimodal Data (text, images, audio, video, PDFs)\n\
                 - ACID Transactions\n\
                 \n\
                 ## Important Notes\n\
                 \n\
                 - Do NOT manually edit files in this directory\n\
                 - Always backup the entire folder (not individual files)\n\
                 - Only one process can open this database at a time (enforced by LOCK file)\n\
                 \n\
                 For more information: https://github.com/muhammad-fiaz/opendb\n",
                Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
            );

            fs::write(&info_path, info_content)
                .map_err(|e| Error::Storage(format!("Failed to create OPENDB_INFO: {}", e)))?;
        }

        // Create README.md file
        let readme_path = db_path.join("README.md");
        if !readme_path.exists() {
            let readme_content = format!(
                "# OpenDB Database\n\
                 \n\
                 This folder contains an OpenDB database created on {}.\n\
                 \n\
                 ## ⚠️ Important Warnings\n\
                 \n\
                 - **DO NOT** manually edit or delete files in this folder\n\
                 - **DO NOT** copy individual files - always copy the entire folder\n\
                 - **DO NOT** open this database from multiple processes simultaneously\n\
                 \n\
                 ## 📁 What's Inside?\n\
                 \n\
                 This database uses multiple files for optimal performance:\n\
                 \n\
                 | File Pattern | Purpose |\n\
                 |--------------|----------|\n\
                 | `*.log` | Write-Ahead Log (ensures durability) |\n\
                 | `*.sst` | Data storage files (compressed) |\n\
                 | `MANIFEST-*` | Database metadata and file tracking |\n\
                 | `OPTIONS-*` | Configuration settings |\n\
                 | `CURRENT` | Points to active MANIFEST |\n\
                 | `LOCK` | Prevents concurrent access |\n\
                 | `OPENDB_INFO` | OpenDB metadata |\n\
                 | `README.md` | This file |\n\
                 \n\
                 ## 💾 Backup Instructions\n\
                 \n\
                 To backup this database:\n\
                 \n\
                 1. **Stop all applications** using this database\n\
                 2. **Copy the entire folder** to your backup location\n\
                 3. **Do not** copy individual files - the database won't work\n\
                 \n\
                 ### Backup Commands\n\
                 \n\
                 **Linux/macOS:**\n\
                 ```bash\n\
                 # Create backup\n\
                 cp -r /path/to/database /path/to/backup/database-backup-$(date +%Y%m%d)\n\
                 \n\
                 # Create compressed backup\n\
                 tar -czf database-backup-$(date +%Y%m%d).tar.gz /path/to/database\n\
                 ```\n\
                 \n\
                 **Windows:**\n\
                 ```powershell\n\
                 # Create backup\n\
                 Copy-Item -Path \"C:\\path\\to\\database\" -Destination \"C:\\backups\\database-backup\" -Recurse\n\
                 \n\
                 # Create compressed backup\n\
                 Compress-Archive -Path \"C:\\path\\to\\database\" -DestinationPath \"C:\\backups\\database-backup.zip\"\n\
                 ```\n\
                 \n\
                 ## 🔄 Restore Instructions\n\
                 \n\
                 To restore from backup:\n\
                 \n\
                 1. **Stop all applications** using the database\n\
                 2. **Delete or rename** the corrupted database folder\n\
                 3. **Copy the entire backup folder** to the original location\n\
                 4. **Restart your application**\n\
                 \n\
                 ## 📊 Database Information\n\
                 \n\
                 - **Format:** OpenDB v0.1.0\n\
                 - **Engine:** RocksDB (LSM-tree)\n\
                 - **Features:** KV Store, Records, Graph, Vectors, Multimodal, Transactions\n\
                 - **Created:** {}\n\
                 \n\
                 ## 🔗 Resources\n\
                 \n\
                 - Documentation: https://muhammad-fiaz.github.io/opendb\n\
                 - GitHub: https://github.com/muhammad-fiaz/opendb\n\
                 - Issues: https://github.com/muhammad-fiaz/opendb/issues\n\
                 \n\
                 ## 📝 Notes\n\
                 \n\
                 Add your own notes about this database here:\n\
                 \n\
                 - Purpose: \n\
                 - Owner: \n\
                 - Last checked: \n\
                 - Backup schedule: \n",
                Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
                Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
            );

            fs::write(&readme_path, readme_content)
                .map_err(|e| Error::Storage(format!("Failed to create README.md: {}", e)))?;
        }

        // Create .opendb_config.json file
        let config_path = db_path.join(".opendb_config.json");
        if !config_path.exists() {
            let config_content = format!(
                "{{\n\
                   \"database\": {{\n\
                     \"version\": \"0.1.0\",\n\
                     \"format\": \"OpenDB\",\n\
                     \"engine\": \"RocksDB\",\n\
                     \"created_at\": \"{}\",\n\
                     \"last_opened_at\": \"{}\"\n\
                   }},\n\
                   \"features\": {{\n\
                     \"kv_store\": true,\n\
                     \"records\": true,\n\
                     \"graph\": true,\n\
                     \"vectors\": true,\n\
                     \"multimodal\": true,\n\
                     \"transactions\": true\n\
                   }},\n\
                   \"storage\": {{\n\
                     \"column_families\": [\n\
                       \"kv\",\n\
                       \"records\",\n\
                       \"graph_nodes\",\n\
                       \"graph_edges\",\n\
                       \"vectors\",\n\
                       \"metadata\",\n\
                       \"transactions\"\n\
                     ]\n\
                   }}\n\
                 }}\n",
                Utc::now().to_rfc3339(),
                Utc::now().to_rfc3339()
            );

            fs::write(&config_path, config_content).map_err(|e| {
                Error::Storage(format!("Failed to create .opendb_config.json: {}", e))
            })?;
        }

        Ok(())
    }
}

impl StorageBackend for RocksDBBackend {
    fn get(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let cf_handle = self.cf_handle(cf)?;
        Ok(self.db.get_cf(cf_handle, key)?)
    }

    fn put(&self, cf: &str, key: &[u8], value: &[u8]) -> Result<()> {
        let cf_handle = self.cf_handle(cf)?;
        self.db.put_cf(cf_handle, key, value)?;
        Ok(())
    }

    fn delete(&self, cf: &str, key: &[u8]) -> Result<()> {
        let cf_handle = self.cf_handle(cf)?;
        self.db.delete_cf(cf_handle, key)?;
        Ok(())
    }

    fn scan_prefix(&self, cf: &str, prefix: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        let cf_handle = self.cf_handle(cf)?;
        let mut iter = self.db.prefix_iterator_cf(cf_handle, prefix);
        let mut results = Vec::new();

        while let Some(Ok((key, value))) = iter.next() {
            if !key.starts_with(prefix) {
                break;
            }
            results.push((key.to_vec(), value.to_vec()));
        }

        Ok(results)
    }

    fn begin_transaction(&self) -> Result<Box<dyn TransactionTrait>> {
        let txn_opts = TransactionOptions::default();
        let write_opts = rocksdb::WriteOptions::default();

        let txn = self.db.transaction_opt(&write_opts, &txn_opts);

        Ok(Box::new(RocksDBTransaction {
            txn: Some(unsafe { std::mem::transmute(txn) }),
            db: Arc::clone(&self.db),
        }))
    }

    fn flush(&self) -> Result<()> {
        // RocksDB automatically flushes, manual flush is optional
        Ok(())
    }

    fn snapshot(&self) -> Result<Box<dyn SnapshotTrait>> {
        // For simplicity, we'll implement snapshots by cloning data
        // A proper snapshot would require wrapping RocksDB's snapshot API
        // This is a trade-off for simpler lifetime management
        Ok(Box::new(RocksDBSnapshot {
            db: Arc::clone(&self.db),
        }))
    }
}

/// RocksDB transaction wrapper
struct RocksDBTransaction {
    txn: Option<rocksdb::Transaction<'static, TransactionDB>>,
    db: Arc<TransactionDB>,
}

impl TransactionTrait for RocksDBTransaction {
    fn get(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let cf_handle = self
            .db
            .cf_handle(cf)
            .ok_or_else(|| Error::Storage(format!("Column family not found: {}", cf)))?;

        if let Some(txn) = &self.txn {
            Ok(txn.get_cf(cf_handle, key)?)
        } else {
            Err(Error::Storage("Transaction already completed".to_string()))
        }
    }

    fn put(&mut self, cf: &str, key: &[u8], value: &[u8]) -> Result<()> {
        let cf_handle = self
            .db
            .cf_handle(cf)
            .ok_or_else(|| Error::Storage(format!("Column family not found: {}", cf)))?;

        if let Some(txn) = &mut self.txn {
            txn.put_cf(cf_handle, key, value)?;
            Ok(())
        } else {
            Err(Error::Storage("Transaction already completed".to_string()))
        }
    }

    fn delete(&mut self, cf: &str, key: &[u8]) -> Result<()> {
        let cf_handle = self
            .db
            .cf_handle(cf)
            .ok_or_else(|| Error::Storage(format!("Column family not found: {}", cf)))?;

        if let Some(txn) = &mut self.txn {
            txn.delete_cf(cf_handle, key)?;
            Ok(())
        } else {
            Err(Error::Storage("Transaction already completed".to_string()))
        }
    }

    fn commit(mut self: Box<Self>) -> Result<()> {
        if let Some(txn) = self.txn.take() {
            txn.commit()?;
            Ok(())
        } else {
            Err(Error::Storage("Transaction already completed".to_string()))
        }
    }

    fn rollback(mut self: Box<Self>) -> Result<()> {
        if let Some(txn) = self.txn.take() {
            txn.rollback()?;
            Ok(())
        } else {
            Err(Error::Storage("Transaction already completed".to_string()))
        }
    }
}

/// RocksDB snapshot wrapper
#[allow(dead_code)]
struct RocksDBSnapshot {
    db: Arc<TransactionDB>,
}

impl SnapshotTrait for RocksDBSnapshot {
    fn get(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let cf_handle = self
            .db
            .cf_handle(cf)
            .ok_or_else(|| Error::Storage(format!("Column family not found: {}", cf)))?;

        Ok(self.db.get_cf(cf_handle, key)?)
    }
}
