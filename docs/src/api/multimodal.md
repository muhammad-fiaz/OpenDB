# Multimodal File Support

OpenDB provides production-ready support for multimodal file processing, designed specifically for AI/LLM applications, RAG (Retrieval Augmented Generation) pipelines, and agent memory systems.

## Overview

The multimodal API enables you to:
- Detect and classify file types (PDF, DOCX, audio, video, text)
- Process and chunk large documents
- Store extracted text with embeddings
- Track processing status for async workflows
- Add custom metadata for any file type

## File Type Detection

### FileType Enum

The `FileType` enum represents supported file formats:

```rust
use opendb::FileType;

// Automatic detection from file extension
let pdf_type = FileType::from_extension("pdf");
assert_eq!(pdf_type, FileType::Pdf);

let audio_type = FileType::from_extension("mp3");
assert_eq!(audio_type, FileType::Audio);

// Get human-readable description
println!("{}", pdf_type.description()); // "PDF document"
println!("{}", audio_type.description()); // "Audio file"
```

### Supported File Types

| FileType | Extensions | Description |
|----------|-----------|-------------|
| `Text` | .txt | Plain text file |
| `Pdf` | .pdf | PDF document |
| `Docx` | .docx | Microsoft Word document |
| `Audio` | .mp3, .wav, .ogg, .flac | Audio file |
| `Video` | .mp4, .avi, .mkv, .mov | Video file |
| `Image` | .jpg, .png, .gif, .bmp | Image file |
| `Unknown` | others | Unknown file type |

### Example: File Type Detection

```rust
use opendb::FileType;

fn detect_file_type(filename: &str) -> FileType {
    let extension = filename
        .rsplit('.')
        .next()
        .unwrap_or("");
    
    FileType::from_extension(extension)
}

// Usage
let file = "research_paper.pdf";
let file_type = detect_file_type(file);

match file_type {
    FileType::Pdf => println!("Processing PDF document"),
    FileType::Audio => println!("Transcribing audio file"),
    FileType::Video => println!("Extracting video captions"),
    _ => println!("Unsupported file type"),
}
```

## Multimodal Documents

### MultimodalDocument Structure

The `MultimodalDocument` struct represents a processed file with extracted content:

```rust
pub struct MultimodalDocument {
    pub id: String,
    pub filename: String,
    pub file_type: FileType,
    pub file_size: usize,
    pub extracted_text: String,
    pub chunks: Vec<DocumentChunk>,
    pub embedding: Option<Vec<f32>>,
    pub metadata: HashMap<String, String>,
    pub processing_status: ProcessingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### CRUD Operations

#### Create

```rust
use opendb::{MultimodalDocument, FileType};

// Create a new multimodal document
let doc = MultimodalDocument::new(
    "doc_001",                     // Unique ID
    "research_paper.pdf",          // Filename
    FileType::Pdf,                 // File type
    1024 * 500,                    // File size in bytes (500 KB)
    "Extracted text content...",   // Extracted text
    vec![0.1; 384],                // Document embedding (384-dim)
);

// Add metadata
let doc = doc
    .with_metadata("author", "Dr. Jane Smith")
    .with_metadata("pages", "25")
    .with_metadata("year", "2024")
    .with_metadata("category", "machine-learning");

println!("Created document: {}", doc.id);
println!("Status: {:?}", doc.processing_status);
```

#### Read

```rust
// Access document properties
println!("Filename: {}", doc.filename);
println!("File type: {:?}", doc.file_type);
println!("File size: {} KB", doc.file_size / 1024);
println!("Extracted text length: {} chars", doc.extracted_text.len());
println!("Number of chunks: {}", doc.chunks.len());

// Access metadata
if let Some(author) = doc.metadata.get("author") {
    println!("Author: {}", author);
}

// Check processing status
match &doc.processing_status {
    ProcessingStatus::Completed => println!("✓ Processing complete"),
    ProcessingStatus::Processing => println!("⏳ Still processing..."),
    ProcessingStatus::Failed(err) => println!("✗ Failed: {}", err),
    ProcessingStatus::Queued => println!("⏸ Queued for processing"),
}
```

#### Update

```rust
use opendb::ProcessingStatus;

// Update processing status
let mut doc = doc.clone();
doc.processing_status = ProcessingStatus::Processing;

// Add more metadata
doc.metadata.insert("processed_by".to_string(), "worker-01".to_string());
doc.metadata.insert("processing_time_ms".to_string(), "1234".to_string());

// Mark as completed
doc.processing_status = ProcessingStatus::Completed;
doc.updated_at = chrono::Utc::now();

println!("Updated document: {}", doc.id);
```

#### Delete

```rust
// In OpenDB, you would typically delete by ID using the database handle
// This is a conceptual example showing how to remove from memory

let mut documents: Vec<MultimodalDocument> = vec![/* ... */];
documents.retain(|d| d.id != "doc_001");

println!("Document deleted");
```

## Document Chunking

### DocumentChunk Structure

For large documents, use `DocumentChunk` to split content into processable segments:

```rust
pub struct DocumentChunk {
    pub chunk_id: String,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
    pub start_offset: usize,
    pub end_offset: usize,
    pub metadata: HashMap<String, String>,
}
```

### Creating Chunks

```rust
use opendb::{DocumentChunk, MultimodalDocument};

let mut doc = MultimodalDocument::new(
    "doc_002",
    "large_book.pdf",
    FileType::Pdf,
    1024 * 1024 * 5, // 5 MB
    "Full book content...",
    vec![0.1; 384],
);

// Add chunks (e.g., by chapter or page)
doc.add_chunk(DocumentChunk::new(
    "chunk_0",
    "Chapter 1: Introduction to Rust programming...",
    vec![0.15; 384],  // Chunk-specific embedding
    0,                // Start offset
    1500,             // End offset
).with_metadata("chapter", "1")
  .with_metadata("page_start", "1")
  .with_metadata("page_end", "15"));

doc.add_chunk(DocumentChunk::new(
    "chunk_1",
    "Chapter 2: Ownership and Borrowing...",
    vec![0.25; 384],
    1500,
    3200,
).with_metadata("chapter", "2")
  .with_metadata("page_start", "16")
  .with_metadata("page_end", "32"));

println!("Added {} chunks", doc.chunks.len());
```

### Chunk Strategies

#### 1. Fixed-Size Chunking

```rust
fn chunk_by_size(text: &str, chunk_size: usize) -> Vec<String> {
    text.chars()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

// Usage
let text = "Very long document text...";
let chunks = chunk_by_size(&text, 1000);
```

#### 2. Paragraph-Based Chunking

```rust
fn chunk_by_paragraphs(text: &str, max_paragraphs: usize) -> Vec<String> {
    text.split("\n\n")
        .collect::<Vec<_>>()
        .chunks(max_paragraphs)
        .map(|chunk| chunk.join("\n\n"))
        .collect()
}

// Usage
let chunks = chunk_by_paragraphs(&text, 3);
```

#### 3. Token-Based Chunking (for LLMs)

```rust
// Requires tiktoken-rs or similar tokenizer
fn chunk_by_tokens(text: &str, max_tokens: usize) -> Vec<String> {
    // Pseudo-code - use actual tokenizer in production
    let tokens = tokenize(text);
    tokens
        .chunks(max_tokens)
        .map(|chunk| detokenize(chunk))
        .collect()
}
```

## Processing Status

### ProcessingStatus Enum

Track the lifecycle of document processing:

```rust
use opendb::ProcessingStatus;

// Status variants
let queued = ProcessingStatus::Queued;
let processing = ProcessingStatus::Processing;
let completed = ProcessingStatus::Completed;
let failed = ProcessingStatus::Failed("OCR error".to_string());

// Pattern matching
match doc.processing_status {
    ProcessingStatus::Queued => {
        println!("Document is queued for processing");
    }
    ProcessingStatus::Processing => {
        println!("Processing in progress...");
    }
    ProcessingStatus::Completed => {
        println!("✓ Processing completed successfully");
    }
    ProcessingStatus::Failed(error) => {
        eprintln!("✗ Processing failed: {}", error);
    }
}
```

## Production Workflow

### Complete PDF Processing Example

```rust
use opendb::{OpenDB, MultimodalDocument, DocumentChunk, FileType, ProcessingStatus};
use std::fs;

fn process_pdf(filepath: &str, db: &OpenDB) -> Result<String> {
    // 1. Read file
    let file_bytes = fs::read(filepath)?;
    let filename = filepath.rsplit('/').next().unwrap();
    
    // 2. Extract text (use pdf-extract or pdfium in production)
    let extracted_text = extract_pdf_text(&file_bytes)?;
    
    // 3. Generate document embedding
    let doc_embedding = generate_embedding(&extracted_text)?;
    
    // 4. Create multimodal document
    let mut doc = MultimodalDocument::new(
        &generate_id(),
        filename,
        FileType::Pdf,
        file_bytes.len(),
        &extracted_text,
        doc_embedding,
    )
    .with_metadata("source", "upload")
    .with_metadata("pages", &count_pages(&file_bytes).to_string());
    
    // 5. Chunk the document
    let chunks = chunk_text(&extracted_text, 1000);
    for (i, chunk_text) in chunks.iter().enumerate() {
        let chunk_embedding = generate_embedding(chunk_text)?;
        let chunk = DocumentChunk::new(
            &format!("chunk_{}", i),
            chunk_text,
            chunk_embedding,
            i * 1000,
            (i + 1) * 1000,
        )
        .with_metadata("chunk_index", &i.to_string());
        
        doc.add_chunk(chunk);
    }
    
    // 6. Mark as completed
    doc.processing_status = ProcessingStatus::Completed;
    
    // 7. Store in OpenDB (pseudo-code - actual storage via Memory type)
    let doc_id = doc.id.clone();
    store_document(db, &doc)?;
    
    Ok(doc_id)
}

// Helper functions (implement with actual libraries)
fn extract_pdf_text(bytes: &[u8]) -> Result<String> {
    // Use pdf-extract, pdfium, or poppler
    todo!("Implement with pdf-extract crate")
}

fn generate_embedding(text: &str) -> Result<Vec<f32>> {
    // Use sentence-transformers, OpenAI API, or onnxruntime
    todo!("Implement with embedding model")
}

fn chunk_text(text: &str, size: usize) -> Vec<String> {
    // Smart chunking by sentences/paragraphs
    todo!("Implement chunking strategy")
}

fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn count_pages(bytes: &[u8]) -> usize {
    // Parse PDF to count pages
    todo!("Implement page counting")
}

fn store_document(db: &OpenDB, doc: &MultimodalDocument) -> Result<()> {
    // Store document and chunks as Memory records with embeddings
    todo!("Implement storage logic")
}
```

### Audio Transcription Example

```rust
use opendb::{MultimodalDocument, DocumentChunk, FileType, ProcessingStatus};

fn process_audio(filepath: &str) -> Result<MultimodalDocument> {
    let file_bytes = fs::read(filepath)?;
    let filename = filepath.rsplit('/').next().unwrap();
    
    // 1. Transcribe audio (use whisper-rs or OpenAI Whisper API)
    let transcript = transcribe_audio(&file_bytes)?;
    
    // 2. Generate embedding from transcript
    let embedding = generate_embedding(&transcript)?;
    
    // 3. Create multimodal document
    let mut doc = MultimodalDocument::new(
        &generate_id(),
        filename,
        FileType::Audio,
        file_bytes.len(),
        &transcript,
        embedding,
    )
    .with_metadata("duration_seconds", &get_audio_duration(&file_bytes).to_string())
    .with_metadata("transcription_model", "whisper-large-v3");
    
    // 4. Add timestamped chunks
    let timestamped_segments = get_timestamped_segments(&file_bytes)?;
    for (i, segment) in timestamped_segments.iter().enumerate() {
        let chunk_embedding = generate_embedding(&segment.text)?;
        let chunk = DocumentChunk::new(
            &format!("segment_{}", i),
            &segment.text,
            chunk_embedding,
            segment.start_offset,
            segment.end_offset,
        )
        .with_metadata("timestamp_start", &segment.start_time.to_string())
        .with_metadata("timestamp_end", &segment.end_time.to_string());
        
        doc.add_chunk(chunk);
    }
    
    doc.processing_status = ProcessingStatus::Completed;
    Ok(doc)
}

struct AudioSegment {
    text: String,
    start_time: f64,
    end_time: f64,
    start_offset: usize,
    end_offset: usize,
}

fn transcribe_audio(bytes: &[u8]) -> Result<String> {
    // Use whisper-rs or cloud API
    todo!("Implement transcription")
}

fn get_audio_duration(bytes: &[u8]) -> f64 {
    // Parse audio metadata
    todo!("Implement duration extraction")
}

fn get_timestamped_segments(bytes: &[u8]) -> Result<Vec<AudioSegment>> {
    // Use Whisper with timestamps
    todo!("Implement segment extraction")
}
```

## Integration with OpenDB

### Storing Multimodal Documents

```rust
use opendb::{OpenDB, Memory, MultimodalDocument};

fn store_multimodal_document(db: &OpenDB, doc: &MultimodalDocument) -> Result<()> {
    // Store main document as Memory
    let memory = Memory::new(
        &doc.id,
        &doc.extracted_text,
        doc.embedding.clone().unwrap_or_default(),
        1.0, // importance
    )
    .with_metadata("filename", &doc.filename)
    .with_metadata("file_type", &format!("{:?}", doc.file_type))
    .with_metadata("file_size", &doc.file_size.to_string());
    
    db.insert_memory(&memory)?;
    
    // Store each chunk as separate Memory with relationships
    for chunk in &doc.chunks {
        let chunk_memory = Memory::new(
            &format!("{}_{}", doc.id, chunk.chunk_id),
            &chunk.content,
            chunk.embedding.clone().unwrap_or_default(),
            0.8, // chunk importance
        )
        .with_metadata("parent_doc", &doc.id)
        .with_metadata("chunk_id", &chunk.chunk_id);
        
        db.insert_memory(&chunk_memory)?;
        
        // Link chunk to parent document
        db.link(&memory.id, "has_chunk", &chunk_memory.id)?;
    }
    
    Ok(())
}
```

### Semantic Search Across Documents

```rust
use opendb::{OpenDB, SearchResult};

fn search_documents(
    db: &OpenDB,
    query: &str,
    top_k: usize,
) -> Result<Vec<SearchResult>> {
    // Generate query embedding
    let query_embedding = generate_embedding(query)?;
    
    // Search across all documents and chunks
    let results = db.search_similar(&query_embedding, top_k)?;
    
    Ok(results)
}

// Usage
let results = search_documents(&db, "machine learning algorithms", 5)?;
for result in results {
    println!("Found: {} (distance: {:.4})", 
             result.memory.content,
             result.distance);
}
```

## Best Practices

### 1. Chunking Strategy
- **Small chunks (500-1000 chars)**: Better precision, more API calls
- **Large chunks (1500-3000 chars)**: More context, fewer API calls
- **Overlap chunks**: 10-20% overlap for continuity

### 2. Metadata Usage
- Always add source file metadata
- Include timestamps for temporal data
- Add processing metadata (model version, date)
- Store original file path for reference

### 3. Error Handling
```rust
use opendb::ProcessingStatus;

fn safe_process(filepath: &str) -> MultimodalDocument {
    let mut doc = MultimodalDocument::new(
        &generate_id(),
        filepath,
        FileType::Unknown,
        0,
        "",
        vec![],
    );
    
    doc.processing_status = ProcessingStatus::Queued;
    
    match process_file(filepath) {
        Ok(processed) => {
            doc = processed;
            doc.processing_status = ProcessingStatus::Completed;
        }
        Err(e) => {
            doc.processing_status = ProcessingStatus::Failed(e.to_string());
            eprintln!("Processing failed: {}", e);
        }
    }
    
    doc
}
```

### 4. Memory Management
- Process files in batches
- Clear processed chunks from memory
- Use streaming for very large files
- Implement backpressure for async processing

## See Also

- [Records Management](records.md) - Storing Memory records
- [Vector Search](vector.md) - Semantic similarity search
- [Graph Operations](graph.md) - Linking documents and chunks
- [Multimodal Example](../../examples/multimodal_agent.rs) - Complete working example

## Production Libraries

### PDF Processing
- `pdf-extract` - Text extraction
- `pdfium-render` - Rendering and OCR
- `lopdf` - Low-level parsing

### DOCX Processing
- `docx-rs` - Read/write DOCX
- `mammoth-rs` - Convert to text

### Audio Transcription
- `whisper-rs` - Local Whisper
- OpenAI Whisper API - Cloud service

### Video Processing
- `ffmpeg-next` - Video/audio extraction
- Combine with whisper for captions

### Embeddings
- `sentence-transformers` (Python + PyO3)
- OpenAI Embeddings API
- `onnxruntime` - Local models
