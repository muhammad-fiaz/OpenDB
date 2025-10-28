// Example: Understanding OpenDB Database Structure
//
// This example creates a database and shows what files are created,
// explaining why OpenDB uses a folder structure instead of a single file.

use colored::*;
use opendb::{Memory, OpenDB, Result};
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    println!(
        "{}",
        "OpenDB Database Structure Example".bright_cyan().bold()
    );
    println!("{}", "==================================".bright_cyan());
    println!();

    // Clean up any existing database
    let db_path = "./data/structure_demo";
    if Path::new(db_path).exists() {
        fs::remove_dir_all(db_path).ok();
    }

    println!("{}", "Creating a new OpenDB database...".bright_yellow());
    println!();

    // Create database
    let db = OpenDB::open(db_path)?;

    println!(
        "{} Database created at: {}",
        "✓".green(),
        db_path.bright_white()
    );
    println!();

    // Insert some data
    println!("{}", "Inserting sample data...".bright_yellow());
    for i in 0..5 {
        let memory = Memory::new(
            &format!("demo_{}", i),
            &format!("Sample content {}", i),
            vec![0.1 * i as f32; 384],
            0.5 + (i as f32 * 0.1),
        );
        db.insert_memory(&memory)?;
    }
    println!("{} Inserted 5 memory records", "✓".green());
    println!();

    // Show database structure
    println!("{}", "Database Folder Contents:".bright_cyan().bold());
    println!("{}", "========================".bright_cyan());
    println!();

    if let Ok(entries) = fs::read_dir(db_path) {
        let mut files: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        files.sort_by_key(|e| e.file_name());

        for entry in files {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            let metadata = entry.metadata().ok();
            let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);

            // Color-code different file types
            let (icon, description) = match file_name_str.as_ref() {
                "OPENDB_INFO" => (
                    "📄".to_string(),
                    format!("OpenDB metadata file ({} bytes)", size).bright_green(),
                ),
                "CURRENT" => (
                    "🔗".to_string(),
                    format!("Points to active MANIFEST ({} bytes)", size).yellow(),
                ),
                "IDENTITY" => (
                    "🆔".to_string(),
                    format!("Database unique ID ({} bytes)", size).cyan(),
                ),
                "LOCK" => (
                    "🔒".to_string(),
                    format!("Prevents concurrent access ({} bytes)", size).bright_red(),
                ),
                s if s.starts_with("MANIFEST") => (
                    "📋".to_string(),
                    format!("Database file list ({} bytes)", size).bright_magenta(),
                ),
                s if s.starts_with("OPTIONS") => (
                    "⚙️".to_string(),
                    format!("RocksDB settings ({} bytes)", size).blue(),
                ),
                s if s.ends_with(".log") => (
                    "📝".to_string(),
                    format!("Write-Ahead Log ({} bytes)", size).bright_yellow(),
                ),
                s if s.ends_with(".sst") => (
                    "💾".to_string(),
                    format!("Data storage file ({} bytes)", size).bright_white(),
                ),
                s if s.starts_with("LOG") => (
                    "📊".to_string(),
                    format!("RocksDB log file ({} bytes)", size).white(),
                ),
                _ => (
                    "📁".to_string(),
                    format!("Other file ({} bytes)", size).white(),
                ),
            };

            println!(
                "  {} {} - {}",
                icon,
                file_name_str.bright_white(),
                description
            );
        }
    }

    println!();
    println!("{}", "File Explanations:".bright_cyan().bold());
    println!("{}", "==================".bright_cyan());
    println!();

    println!("{}", "📄 OPENDB_INFO".bright_green().bold());
    println!("   OpenDB-specific metadata file (created by OpenDB)");
    println!("   Explains database format and features");
    println!();

    println!("{}", "📝 *.log files (WAL)".bright_yellow().bold());
    println!("   Write-Ahead Log for durability");
    println!("   Changes written here first, then flushed to SST files");
    println!();

    println!("{}", "💾 *.sst files".bright_white().bold());
    println!("   Sorted String Tables - actual data storage");
    println!("   Compressed and immutable once created");
    println!();

    println!("{}", "📋 MANIFEST-*".bright_magenta().bold());
    println!("   Tracks active SST files and database structure");
    println!();

    println!("{}", "🔒 LOCK".bright_red().bold());
    println!("   Ensures only one process accesses the database");
    println!();

    println!("{}", "Why Folder-Based Architecture?".bright_cyan().bold());
    println!("{}", "==============================".bright_cyan());
    println!();

    println!(
        "  {} {}",
        "✅".green(),
        "Higher write throughput (sequential WAL writes)"
    );
    println!(
        "  {} {}",
        "✅".green(),
        "Better compression (data compressed in SST files)"
    );
    println!(
        "  {} {}",
        "✅".green(),
        "Efficient compaction (background merging)"
    );
    println!(
        "  {} {}",
        "✅".green(),
        "Crash recovery (WAL enables reliable recovery)"
    );
    println!(
        "  {} {}",
        "✅".green(),
        "Horizontal scaling (easier to distribute files)"
    );
    println!();

    println!("{}", "Important Notes:".bright_red().bold());
    println!("{}", "===============".bright_red());
    println!();
    println!(
        "  {} Always backup the {} (not individual files)",
        "⚠️".yellow(),
        "entire folder".bright_white().underline()
    );
    println!(
        "  {} Do NOT manually edit files in the database folder",
        "⚠️".yellow()
    );
    println!(
        "  {} Only one process can open a database at a time",
        "⚠️".yellow()
    );
    println!();

    println!("{}", "To view database documentation:".bright_cyan());
    println!("  {} cat {}/OPENDB_INFO", "$".bright_white(), db_path);
    println!("  {} cat {}/README.md", "$".bright_white(), db_path);
    println!(
        "  {} cat {}/.opendb_config.json",
        "$".bright_white(),
        db_path
    );
    println!();

    drop(db);

    println!("{} Example complete!", "✨".bright_green());
    println!();

    Ok(())
}
