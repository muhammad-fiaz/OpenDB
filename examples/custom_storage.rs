// Example: Custom Database Storage Location
//
// This example demonstrates how to configure custom storage paths
// for OpenDB, useful for production deployments.

use colored::*;
use opendb::{Memory, OpenDB, OpenDBOptions, Result};
use std::env;

fn main() -> Result<()> {
    println!(
        "{}",
        "OpenDB Custom Storage Location Example"
            .bright_cyan()
            .bold()
    );
    println!(
        "{}",
        "========================================".bright_cyan()
    );
    println!();

    println!(
        "{}",
        "Note: OpenDB creates a folder (not a single file) with multiple files inside."
            .bright_magenta()
    );
    println!(
        "{}",
        "Check the OPENDB_INFO file in any database folder for details.".bright_magenta()
    );
    println!();

    // Example 1: Default storage location
    println!("{}", "1️⃣  Default Storage Location".bright_yellow());
    let db1 = OpenDB::open("./data/default_db")?;
    println!(
        "  {} Database folder created at: {}",
        "✓".green(),
        "./data/default_db/".bright_white()
    );
    println!(
        "  {} Contains: OPENDB_INFO, *.log, *.sst, MANIFEST, etc.",
        "📁".cyan()
    );
    drop(db1);

    println!();

    // Example 2: Custom storage with options
    println!("{}", "2️⃣  Custom Storage via Options".bright_yellow());
    let custom_path = "./data/custom_location";
    let options = OpenDBOptions::new()
        .with_storage_path(custom_path)
        .with_kv_cache_size(2000)
        .with_record_cache_size(1000)
        .dimension(384);

    let db2 = OpenDB::open_with_options("./data/original_path", options)?;
    println!(
        "  {} Database configured with custom path: {}",
        "✓".green(),
        custom_path.bright_white()
    );

    // Test the database
    let memory = Memory::new(
        "test_1",
        "Testing custom storage location",
        vec![0.1; 384],
        0.9,
    );
    db2.insert_memory(&memory)?;
    println!("  {} Test memory inserted successfully", "✓".green());
    drop(db2);

    println!();

    // Example 3: Environment-based storage (production pattern)
    println!(
        "{}",
        "3️⃣  Environment-based Storage (Production)".bright_yellow()
    );
    let db_path = env::var("OPENDB_PATH").unwrap_or_else(|_| "./data/production_db".to_string());

    let prod_options = OpenDBOptions::with_dimension(768) // Larger embeddings for production
        .with_kv_cache_size(5000)
        .with_record_cache_size(3000);

    let db3 = OpenDB::open_with_options(&db_path, prod_options)?;
    println!(
        "  {} Production database at: {}",
        "✓".green(),
        db_path.bright_white()
    );
    println!("  {} Vector dimension: {}", "✓".green(), "768".cyan());
    println!("  {} KV cache size: {}", "✓".green(), "5000".cyan());
    println!("  {} Record cache size: {}", "✓".green(), "3000".cyan());
    drop(db3);

    println!();

    // Example 4: Multiple databases (tenant isolation pattern)
    println!("{}", "4️⃣  Multi-Tenant Pattern".bright_yellow());
    let tenants = vec!["tenant_a", "tenant_b", "tenant_c"];

    for tenant in tenants {
        let tenant_path = format!("./data/tenants/{}", tenant);
        let tenant_options = OpenDBOptions::with_dimension(384).with_kv_cache_size(1000);

        let db = OpenDB::open_with_options(&tenant_path, tenant_options)?;
        println!(
            "  {} Tenant database: {} at {}",
            "✓".green(),
            tenant.cyan(),
            tenant_path.bright_white()
        );

        // Insert tenant-specific data
        let memory = Memory::new(
            "tenant_data",
            &format!("Data for {}", tenant),
            vec![0.1; 384],
            0.8,
        );
        db.insert_memory(&memory)?;
        drop(db);
    }

    println!();

    // Example 5: Platform-specific paths
    println!("{}", "5️⃣  Platform-Specific Storage Paths".bright_yellow());

    #[cfg(target_os = "windows")]
    let platform_path = "C:\\ProgramData\\OpenDB\\data";

    #[cfg(target_os = "linux")]
    let platform_path = "/var/lib/opendb/data";

    #[cfg(target_os = "macos")]
    let platform_path = "/usr/local/var/opendb/data";

    println!(
        "  {} Recommended path for this platform:",
        "💡".bright_yellow()
    );
    println!("    {}", platform_path.bright_white());

    println!();

    // Best practices summary
    println!(
        "{}",
        "📚 Storage Location Best Practices:".bright_cyan().bold()
    );
    println!();

    println!("{}", "  Development:".bright_white());
    println!(
        "    {} Use relative paths like {}",
        "•".yellow(),
        "./data/dev_db".cyan()
    );
    println!("    {} Keep databases in project directory", "•".yellow());
    println!();

    println!("{}", "  Production:".bright_white());
    println!(
        "    {} Use environment variables for configuration",
        "•".yellow()
    );
    println!("    {} Choose platform-specific paths:", "•".yellow());
    println!(
        "      {} Linux: {}",
        "→".bright_blue(),
        "/var/lib/opendb".cyan()
    );
    println!(
        "      {} macOS: {}",
        "→".bright_blue(),
        "/usr/local/var/opendb".cyan()
    );
    println!(
        "      {} Windows: {}",
        "→".bright_blue(),
        "C:\\ProgramData\\OpenDB".cyan()
    );
    println!();

    println!("{}", "  Multi-Tenant:".bright_white());
    println!(
        "    {} Isolate each tenant in separate database",
        "•".yellow()
    );
    println!(
        "    {} Use pattern: {}",
        "•".yellow(),
        "./data/tenants/<tenant_id>".cyan()
    );
    println!("    {} Consider tenant-specific cache sizes", "•".yellow());
    println!();

    println!("{}", "  Backup & Recovery:".bright_white());
    println!("    {} Store databases on backed-up volumes", "•".yellow());
    println!("    {} Use absolute paths for critical data", "•".yellow());
    println!("    {} Document storage locations in config", "•".yellow());
    println!();

    println!("{}", "  Performance:".bright_white());
    println!("    {} Use SSD storage for best performance", "•".yellow());
    println!("    {} Adjust cache sizes based on workload", "•".yellow());
    println!("    {} Monitor disk usage and I/O", "•".yellow());

    println!();
    println!("{} Configuration complete!", "✅".green());
    println!(
        "{} For more info, visit: {}",
        "💡".bright_yellow(),
        "https://muhammad-fiaz.github.io/opendb"
            .bright_blue()
            .underline()
    );

    Ok(())
}
