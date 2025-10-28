// Core data types for OpenDB
//
// This module defines the primary data structures used in OpenDB.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::Utc;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Memory record - the primary data structure for agent memory storage
///
/// A Memory represents a piece of information with semantic embedding,
/// importance scoring, and arbitrary metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Archive, RkyvSerialize, RkyvDeserialize)]
#[archive(check_bytes)]
pub struct Memory {
    /// Unique identifier for this memory
    pub id: String,

    /// The actual content/text of the memory
    pub content: String,

    /// Vector embedding for semantic search
    pub embedding: Vec<f32>,

    /// Importance score (0.0 to 1.0)
    pub importance: f32,

    /// Creation/update timestamp
    pub timestamp: i64,

    /// Arbitrary key-value metadata
    pub metadata: HashMap<String, String>,
}

impl Memory {
    /// Create a new Memory record
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `content` - Text content
    /// * `embedding` - Vector embedding (typically from an embedding model)
    /// * `importance` - Importance score (0.0 to 1.0)
    ///
    /// # Example
    ///
    /// ```
    /// use opendb::Memory;
    ///
    /// let memory = Memory::new(
    ///     "mem_001",
    ///     "The user prefers dark mode",
    ///     vec![0.1, 0.2, 0.3],
    ///     0.8,
    /// );
    /// ```
    pub fn new(id: impl Into<String>, content: impl Into<String>, embedding: Vec<f32>, importance: f32) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            embedding,
            importance: importance.clamp(0.0, 1.0),
            timestamp: Utc::now().timestamp(),
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to this memory
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Update the timestamp to now
    pub fn touch(&mut self) {
        self.timestamp = Utc::now().timestamp();
    }
}

/// Metadata associated with Memory records
pub type MemoryMetadata = HashMap<String, String>;

/// Graph edge representing a relationship between two entities
#[derive(Debug, Clone, Serialize, Deserialize, Archive, RkyvSerialize, RkyvDeserialize)]
#[archive(check_bytes)]
pub struct Edge {
    /// Source node ID
    pub from: String,

    /// Relationship type
    pub relation: String,

    /// Target node ID
    pub to: String,

    /// Edge weight/strength
    pub weight: f32,

    /// Creation timestamp
    pub timestamp: i64,
}

impl Edge {
    /// Create a new edge
    pub fn new(from: impl Into<String>, relation: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            relation: relation.into(),
            to: to.into(),
            weight: 1.0,
            timestamp: Utc::now().timestamp(),
        }
    }

    /// Create an edge with a specific weight
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }
}

/// Search result with distance score
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Memory ID
    pub id: String,

    /// Distance score (lower is more similar)
    pub distance: f32,

    /// The memory record itself
    pub memory: Memory,
}

// ==============================================================================
// Multimodal File Support for AI/LLM Systems
// ==============================================================================

/// Supported file types for multimodal embeddings and AI processing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Archive, RkyvSerialize, RkyvDeserialize)]
#[archive(check_bytes)]
pub enum FileType {
    /// Plain text file (.txt, .md, etc.)
    Text,
    /// PDF document (.pdf)
    Pdf,
    /// Microsoft Word document (.docx)
    Docx,
    /// Audio file (.mp3, .wav, .m4a, etc.)
    Audio,
    /// Video file (.mp4, .avi, .mkv, etc.)
    Video,
    /// Image file (.jpg, .png, .webp, etc.)
    Image,
    /// Unknown or unsupported file type
    Unknown,
}

impl FileType {
    /// Detect file type from file extension
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "txt" | "md" | "markdown" | "text" => FileType::Text,
            "pdf" => FileType::Pdf,
            "doc" | "docx" => FileType::Docx,
            "mp3" | "wav" | "m4a" | "aac" | "flac" | "ogg" => FileType::Audio,
            "mp4" | "avi" | "mkv" | "mov" | "webm" | "flv" => FileType::Video,
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" => FileType::Image,
            _ => FileType::Unknown,
        }
    }

    /// Get file type description for error messages
    pub fn description(&self) -> &'static str {
        match self {
            FileType::Text => "Plain text file",
            FileType::Pdf => "PDF document",
            FileType::Docx => "Microsoft Word document",
            FileType::Audio => "Audio file",
            FileType::Video => "Video file",
            FileType::Image => "Image file",
            FileType::Unknown => "Unknown file type",
        }
    }
}

/// Multimodal document with extracted content and embeddings
///
/// This structure supports AI/LLM memory systems by storing:
/// - Original file metadata and binary content
/// - Extracted text (from PDF/DOCX/audio transcription/video captions)
/// - Vector embeddings for semantic search
/// - Chunked content for large documents
///
/// # Use Cases
/// - Document question-answering with RAG (Retrieval Augmented Generation)
/// - Multimodal AI agent memory (text, audio, video, images)
/// - Semantic search across diverse content types
/// - Knowledge base construction from various file formats
#[derive(Debug, Clone, Serialize, Deserialize, Archive, RkyvSerialize, RkyvDeserialize)]
#[archive(check_bytes)]
pub struct MultimodalDocument {
    /// Unique document identifier
    pub id: String,

    /// Original filename
    pub filename: String,

    /// File type
    pub file_type: FileType,

    /// Original file size in bytes
    pub file_size: usize,

    /// Extracted text content (from OCR, transcription, or direct extraction)
    pub extracted_text: String,

    /// Document chunks for large files (optional)
    ///
    /// For large documents, content is split into chunks with separate embeddings
    /// to enable efficient semantic search and retrieval
    pub chunks: Vec<DocumentChunk>,

    /// Document-level embedding (summary embedding)
    pub embedding: Vec<f32>,

    /// Creation/ingestion timestamp
    pub timestamp: i64,

    /// Additional metadata (e.g., author, title, transcription model, etc.)
    pub metadata: HashMap<String, String>,
}

impl MultimodalDocument {
    /// Create a new multimodal document
    pub fn new(
        id: impl Into<String>,
        filename: impl Into<String>,
        file_type: FileType,
        file_size: usize,
        extracted_text: impl Into<String>,
        embedding: Vec<f32>,
    ) -> Self {
        Self {
            id: id.into(),
            filename: filename.into(),
            file_type,
            file_size,
            extracted_text: extracted_text.into(),
            chunks: Vec::new(),
            embedding,
            timestamp: Utc::now().timestamp(),
            metadata: HashMap::new(),
        }
    }

    /// Add a chunk to this document
    pub fn add_chunk(&mut self, chunk: DocumentChunk) {
        self.chunks.push(chunk);
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Document chunk for large multimodal files
///
/// Large documents are split into chunks for efficient retrieval and processing:
/// - Each chunk has its own embedding
/// - Chunks can be retrieved individually based on semantic similarity
/// - Supports chunking strategies: sentence-based, token-based, paragraph-based
#[derive(Debug, Clone, Serialize, Deserialize, Archive, RkyvSerialize, RkyvDeserialize)]
#[archive(check_bytes)]
pub struct DocumentChunk {
    /// Chunk identifier (usually index within document)
    pub chunk_id: String,

    /// Chunk text content
    pub content: String,

    /// Chunk embedding
    pub embedding: Vec<f32>,

    /// Start position in original document (character offset)
    pub start_offset: usize,

    /// End position in original document (character offset)
    pub end_offset: usize,

    /// Chunk metadata (e.g., page number, timestamp in video, etc.)
    pub metadata: HashMap<String, String>,
}

impl DocumentChunk {
    /// Create a new document chunk
    pub fn new(
        chunk_id: impl Into<String>,
        content: impl Into<String>,
        embedding: Vec<f32>,
        start_offset: usize,
        end_offset: usize,
    ) -> Self {
        Self {
            chunk_id: chunk_id.into(),
            content: content.into(),
            embedding,
            start_offset,
            end_offset,
            metadata: HashMap::new(),
        }
    }

    /// Add chunk metadata (e.g., page number for PDF, timestamp for video)
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// File processing status for async/batch operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessingStatus {
    /// File is queued for processing
    Queued,
    /// File is currently being processed
    Processing,
    /// File processing completed successfully
    Completed,
    /// File processing failed
    Failed,
}
