// Example: Multimodal AI Agent Memory with File Support
//
// This example demonstrates how to use OpenDB for AI/LLM applications
// with support for various file formats (PDF, DOCX, audio, video, text).

use opendb::{OpenDB, OpenDBOptions, MultimodalDocument, DocumentChunk, FileType, Result};
use colored::*;

fn main() -> Result<()> {
    println!("{}", "OpenDB Multimodal AI Agent Memory Example".bright_cyan().bold());
    println!("{}", "==========================================".bright_cyan());
    println!();

    // Open database with 384-dimensional embeddings (common for AI models)
    let options = OpenDBOptions::with_dimension(384);
    let _db = OpenDB::open_with_options("./data/multimodal_agent", options)?;

    // Example 1: Process a PDF document
    println!("{}", "📄 Processing PDF document...".bright_yellow());
    let pdf_doc = MultimodalDocument::new(
        "doc_001",
        "research_paper.pdf",
        FileType::Pdf,
        1024 * 500, // 500 KB
        "This is extracted text from the PDF. In production, you would use a PDF parsing library like pdf-extract or pdfium.",
        generate_embedding("PDF document about AI research"),
    )
    .with_metadata("author", "Dr. Smith")
    .with_metadata("pages", "15")
    .with_metadata("extraction_method", "pdfium");

    println!("  {} PDF: {} ({} bytes)", "✓".green(), pdf_doc.filename.bright_white(), pdf_doc.file_size);
    println!("  {} Extracted text: {} chars", "✓".green(), pdf_doc.extracted_text.len());

    // Example 2: Process a Word document
    println!();
    println!("{}", "📝 Processing DOCX document...".bright_yellow());
    let docx_doc = MultimodalDocument::new(
        "doc_002",
        "meeting_notes.docx",
        FileType::Docx,
        1024 * 100, // 100 KB
        "Meeting notes extracted from DOCX. Use docx-rs or similar crates for real extraction.",
        generate_embedding("Meeting notes about project planning"),
    )
    .with_metadata("created_by", "Alice")
    .with_metadata("meeting_date", "2024-01-15");

    println!("  {} DOCX: {} ({} bytes)", "✓".green(), docx_doc.filename.bright_white(), docx_doc.file_size);

    // Example 3: Process audio file with transcription
    println!();
    println!("{}", "🎵 Processing audio file...".bright_yellow());
    let mut audio_doc = MultimodalDocument::new(
        "doc_003",
        "podcast_episode.mp3",
        FileType::Audio,
        1024 * 1024 * 50, // 50 MB
        "Transcribed audio content. In production, use whisper-rs or cloud APIs like OpenAI Whisper, Google Speech-to-Text.",
        generate_embedding("Podcast episode about technology trends"),
    )
    .with_metadata("duration_seconds", "3600")
    .with_metadata("transcription_model", "whisper-large-v3");

    // Add timestamped chunks for audio
    audio_doc.add_chunk(DocumentChunk::new(
        "chunk_0",
        "Introduction segment discussing AI trends.",
        generate_embedding("AI trends introduction"),
        0,
        150,
    ).with_metadata("timestamp", "00:00-02:30"));

    audio_doc.add_chunk(DocumentChunk::new(
        "chunk_1",
        "Deep dive into machine learning applications.",
        generate_embedding("Machine learning applications"),
        150,
        450,
    ).with_metadata("timestamp", "02:30-07:30"));

    println!("  {} Audio: {} ({} MB)", "✓".green(), audio_doc.filename.bright_white(), audio_doc.file_size / (1024 * 1024));
    println!("  {} Transcription chunks: {}", "✓".green(), audio_doc.chunks.len());

    // Example 4: Process video file
    println!();
    println!("{}", "🎬 Processing video file...".bright_yellow());
    let mut video_doc = MultimodalDocument::new(
        "doc_004",
        "tutorial.mp4",
        FileType::Video,
        1024 * 1024 * 200, // 200 MB
        "Video captions and frame descriptions. Use ffmpeg + OCR for text extraction.",
        generate_embedding("Tutorial video about Rust programming"),
    )
    .with_metadata("duration_seconds", "1800")
    .with_metadata("resolution", "1920x1080")
    .with_metadata("frame_rate", "30");

    // Add video chunks with frame timestamps
    video_doc.add_chunk(DocumentChunk::new(
        "chunk_0",
        "Introduction to Rust ownership and borrowing.",
        generate_embedding("Rust ownership introduction"),
        0,
        200,
    ).with_metadata("timestamp", "00:00-03:20")
     .with_metadata("frame", "0"));

    println!("  {} Video: {} ({} MB)", "✓".green(), video_doc.filename.bright_white(), video_doc.file_size / (1024 * 1024));

    // Example 5: Plain text file
    println!();
    println!("{}", "📋 Processing text file...".bright_yellow());
    let text_doc = MultimodalDocument::new(
        "doc_005",
        "readme.txt",
        FileType::Text,
        1024 * 10, // 10 KB
        "This is plain text content that can be directly embedded.",
        generate_embedding("README file with project instructions"),
    )
    .with_metadata("encoding", "UTF-8");

    println!("  {} Text: {} ({} KB)", "✓".green(), text_doc.filename.bright_white(), text_doc.file_size / 1024);

    // Demonstrate file type detection
    println!();
    println!("{}", "🔍 File type detection:".bright_magenta());
    let extensions = vec!["pdf", "docx", "mp3", "mp4", "txt", "wav", "avi", "unknown"];
    for ext in extensions {
        let file_type = FileType::from_extension(ext);
        println!("  {} .{}: {} - {}", "•".bright_blue(), ext.yellow(), format!("{:?}", file_type).cyan(), file_type.description());
    }

    // Demonstrate semantic search across multimodal documents
    println!();
    println!("{}", "🔎 Semantic search example:".bright_magenta());
    println!("  {}: '{}'", "Query".bright_white(), "machine learning research".green());
    println!("  This would search across {} document types:", "ALL".bright_red());
    println!("    {} PDF research papers", "•".bright_blue());
    println!("    {} Audio transcriptions", "•".bright_blue());
    println!("    {} Video captions", "•".bright_blue());
    println!("    {} Text notes", "•".bright_blue());
    println!("    {} DOCX meeting notes", "•".bright_blue());

    // Production workflow summary
    println!();
    println!("{}", "📚 Production Workflow for AI/LLM Applications:".bright_cyan().bold());
    println!("  {} Ingest files (PDF, DOCX, audio, video, text)", "1.".bright_white());
    println!("  {} Extract content:", "2.".bright_white());
    println!("     {} PDF: Use pdf-extract, pdfium, or poppler", "•".yellow());
    println!("     {} DOCX: Use docx-rs or mammoth", "•".yellow());
    println!("     {} Audio: Transcribe with whisper-rs or OpenAI Whisper API", "•".yellow());
    println!("     {} Video: Extract captions/audio, use ffmpeg + whisper", "•".yellow());
    println!("     {} Text: Direct reading", "•".yellow());
    println!("  {} Generate embeddings:", "3.".bright_white());
    println!("     {} Use sentence-transformers (e.g., all-MiniLM-L6-v2)", "•".yellow());
    println!("     {} Or OpenAI embeddings API", "•".yellow());
    println!("     {} Or local models via onnx-runtime", "•".yellow());
    println!("  {} Chunk large documents:", "4.".bright_white());
    println!("     {} Split by paragraphs, sentences, or fixed token count", "•".yellow());
    println!("     {} Each chunk gets its own embedding", "•".yellow());
    println!("  {} Store in OpenDB:", "5.".bright_white());
    println!("     {} Use Memory for simple text", "•".yellow());
    println!("     {} Use MultimodalDocument for files", "•".yellow());
    println!("     {} Use DocumentChunk for large file segments", "•".yellow());
    println!("  {} Query with semantic search:", "6.".bright_white());
    println!("     {} Convert user query to embedding", "•".yellow());
    println!("     {} Find similar chunks/documents", "•".yellow());
    println!("     {} Feed relevant context to LLM (RAG pattern)", "•".yellow());

    println!();
    println!("{}", "✨ OpenDB is production-ready for:".bright_green().bold());
    println!("  {} AI agent memory systems", "•".cyan());
    println!("  {} Multimodal RAG (Retrieval Augmented Generation)", "•".cyan());
    println!("  {} Knowledge base construction from diverse sources", "•".cyan());
    println!("  {} Document Q&A systems", "•".cyan());
    println!("  {} Conversational AI with long-term memory", "•".cyan());

    println!();
    println!("{} For help or issues, visit: {}", "💡".bright_yellow(), "https://github.com/muhammad-fiaz/opendb/issues".bright_blue().underline());

    Ok(())
}

/// Generate a dummy embedding for demonstration
///
/// In production, use a real embedding model like:
/// - sentence-transformers (all-MiniLM-L6-v2, all-mpnet-base-v2)
/// - OpenAI text-embedding-ada-002
/// - Cohere embed-multilingual-v3.0
fn generate_embedding(_text: &str) -> Vec<f32> {
    // In production, this would call an actual embedding model
    // For now, return a dummy 384-dimensional vector
    vec![0.0; 384]
}
