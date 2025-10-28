// Error types for OpenDB
//
// This module defines the error types used throughout the database.

use thiserror::Error;

/// Result type alias for OpenDB operations
pub type Result<T> = std::result::Result<T, Error>;

/// GitHub issue URL for error reporting
const GITHUB_ISSUES_URL: &str = "https://github.com/muhammad-fiaz/opendb/issues";

/// Error types that can occur in OpenDB operations
#[derive(Error, Debug)]
pub enum Error {
    /// Storage-layer errors (RocksDB)
    #[error("Storage error: {0}\n\nIf this error persists, please report it at: {GITHUB_ISSUES_URL}")]
    Storage(String),

    /// Serialization/deserialization errors
    #[error("Codec error: {0}\n\nIf this error persists, please report it at: {GITHUB_ISSUES_URL}")]
    Codec(String),

    /// Record not found
    #[error("Record not found: {0}")]
    NotFound(String),

    /// Transaction errors
    #[error("Transaction error: {0}\n\nIf this error persists, please report it at: {GITHUB_ISSUES_URL}")]
    Transaction(String),

    /// Cache errors
    #[error("Cache error: {0}\n\nIf this error persists, please report it at: {GITHUB_ISSUES_URL}")]
    Cache(String),

    /// Vector index errors
    #[error("Vector index error: {0}\n\nIf this error persists, please report it at: {GITHUB_ISSUES_URL}")]
    VectorIndex(String),

    /// Graph errors
    #[error("Graph error: {0}\n\nIf this error persists, please report it at: {GITHUB_ISSUES_URL}")]
    Graph(String),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// I/O errors
    #[error("I/O error: {0}\n\nIf this error persists, please report it at: {GITHUB_ISSUES_URL}")]
    Io(#[from] std::io::Error),

    /// Generic errors
    #[error("Internal error: {0}\n\nThis is likely a bug. Please report it at: {GITHUB_ISSUES_URL}")]
    Internal(String),

    /// Multimodal file processing errors
    #[error("File processing error: {0}\n\nSupported formats: PDF, DOCX, TXT, MP3, MP4, WAV, etc.\nIf you need help, please visit: {GITHUB_ISSUES_URL}")]
    FileProcessing(String),
}

impl Error {
    /// Get the GitHub issues URL for error reporting
    pub fn issues_url() -> &'static str {
        GITHUB_ISSUES_URL
    }

    /// Check if this error should be reported to GitHub
    pub fn should_report(&self) -> bool {
        matches!(
            self,
            Error::Storage(_)
                | Error::Codec(_)
                | Error::Transaction(_)
                | Error::Cache(_)
                | Error::VectorIndex(_)
                | Error::Graph(_)
                | Error::Internal(_)
                | Error::Io(_)
        )
    }

    /// Get a user-friendly error message with reporting instructions
    pub fn user_message(&self) -> String {
        format!(
            "{}\n\n💡 Need help? Visit our GitHub issues: {}",
            self,
            GITHUB_ISSUES_URL
        )
    }
}

impl From<rocksdb::Error> for Error {
    fn from(err: rocksdb::Error) -> Self {
        Error::Storage(err.to_string())
    }
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Internal(err.to_string())
    }
}
