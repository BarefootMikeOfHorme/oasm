/// OASM Scanner CLI
/// Universal pre-compile diagnostic tool
///
/// Usage:
///   oasm-scan <project_root> [--output <dir>]
///   oasm-scan --help

use compiler::scanner::Scanner;
use std::path::PathBuf;
use std::fs;
use anyhow::{Result, Context};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "oasm-scan")]
#[command(about = "Universal codebase scanner for pre-compile diagnostics", long_about = None)]
struct Args {
    /// Project root directory to scan
    #[arg(default_value = ".")]
    root: PathBuf,

    /// Output directory for logs
    #[arg(short, long, default_value = "logs/StructureDebug")]
    output: PathBuf,

    /// Output format (json, yaml, both, dashboard)
    #[arg(short, long, default_value = "dashboard")]
    format: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Enable dashboard format output (JSONL + plain text)
    #[arg(long)]
    dashboard: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Ensure output directory exists
    fs::create_dir_all(&args.output)
        .context("Failed to create output directory")?;

    println!("🔍 OASM Scanner - Pre-Compile Diagnostics");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📂 Root: {}", args.root.display());
    println!("📊 Scanning project structure...\n");

    // Run scan
    let scanner = Scanner::new(&args.root);

    // Dashboard format output
    if args.format == "dashboard" || args.dashboard {
        let dashboard_rows = scanner.scan_with_dashboard()
            .context("Failed to scan with dashboard format")?;

        let timestamp = chrono::Utc::now().format("%Y%m%dT%H%M%S").to_string();

        // Write JSONL (one JSON object per line)
        let jsonl_path = args.output.join(format!("scan_dashboard_{}.jsonl", timestamp));
        let mut jsonl_content = String::new();
        for row in &dashboard_rows {
            if let Ok(json) = row.to_jsonl() {
                jsonl_content.push_str(&json);
                jsonl_content.push('\n');
            }
        }
        fs::write(&jsonl_path, jsonl_content)
            .context("Failed to write JSONL file")?;
        println!("✓ JSONL dashboard: {}", jsonl_path.display());

        // Write plain text dashboard
        let plain_path = args.output.join(format!("scan_dashboard_{}.txt", timestamp));
        let mut plain_content = String::new();
        plain_content.push_str("=== OASM Scan Dashboard ===\n");
        plain_content.push_str(&format!("Timestamp: {}\n", timestamp));
        plain_content.push_str(&format!("Total files: {}\n\n", dashboard_rows.len()));

        for row in &dashboard_rows {
            plain_content.push_str(&row.to_plain_text());
            plain_content.push('\n');
        }

        fs::write(&plain_path, plain_content)
            .context("Failed to write plain text dashboard")?;
        println!("✓ Plain text dashboard: {}", plain_path.display());

        // Also print to stdout
        if args.verbose {
            println!("\n📊 Dashboard Output:");
            for row in &dashboard_rows {
                println!("{}", row.to_plain_text());
            }
        }

        println!("\n✅ Scan complete! ({} files processed)", dashboard_rows.len());
        return Ok(());
    }

    // Original format outputs
    let results = scanner.scan()
        .context("Failed to scan project")?;

    let timestamp = &results.timestamp;

    // Write structured log
    if args.format == "json" || args.format == "both" {
        let json_path = args.output.join(format!("baseline_index_{}.json", timestamp));
        let json = serde_json::to_string_pretty(&results.files)
            .context("Failed to serialize JSON")?;
        fs::write(&json_path, json)
            .context("Failed to write JSON file")?;
        println!("✓ JSON index: {}", json_path.display());
    }

    // Write human-readable log
    if args.format == "both" {
        let log_path = args.output.join(format!("structure_{}.log", timestamp));
        let log_content = format_structure_log(&results);
        fs::write(&log_path, log_content)
            .context("Failed to write structure log")?;
        println!("✓ Structure log: {}", log_path.display());
    }

    // Write CLI state
    let cli_state_path = args.output.join(format!("cli_state_{}.json", timestamp));
    let cli_state = serde_json::to_string_pretty(&results.files)
        .context("Failed to serialize CLI state")?;
    fs::write(&cli_state_path, cli_state)
        .context("Failed to write CLI state")?;
    println!("✓ CLI state: {}", cli_state_path.display());

    // Print summary
    println!("\n📈 Summary:");
    println!("   Files: {}", results.total_files);
    println!("   Total LOC: {}", results.total_loc);
    println!("   Average LOC/file: {}",
        if results.total_files > 0 { results.total_loc / results.total_files } else { 0 });

    // Print top files by LOC
    if args.verbose {
        println!("\n🔝 Top 10 files by LOC:");
        let mut sorted_files = results.files.clone();
        sorted_files.sort_by(|a, b| b.loc.cmp(&a.loc));
        for (i, file) in sorted_files.iter().take(10).enumerate() {
            println!("   {}. {} ({} LOC)", i + 1, file.alias, file.loc);
        }
    }

    println!("\n✅ Scan complete!");
    Ok(())
}

fn format_structure_log(results: &compiler::scanner::StructureLog) -> String {
    let mut output = String::new();

    output.push_str("=== Project Structure Snapshot ===\n");
    output.push_str(&format!("Root: {}\n", results.root));
    output.push_str(&format!("Timestamp: {}\n\n", results.timestamp));
    output.push_str("Format: [n/total] [Stage] [alias] | metrics\n\n");

    for file in &results.files {
        let line = format!(
            " · [{}/{}] {} | {} LOC | {} fn ({} pub, {} unsafe) | Imports={} | Logging: info={} warn={} error={} println={} | Structs={} Enums={} Derives={} | Errors=None | Warnings=None | Tests={} | Modified={}\n",
            file.n,
            results.total_files,
            file.rel_path,
            file.loc,
            file.fn_count,
            file.pub_fn_count,
            file.unsafe_fn_count,
            file.imports,
            file.logging.info,
            file.logging.warn,
            file.logging.error,
            file.logging.println,
            file.structs,
            file.enums,
            file.derives,
            file.tests,
            file.modified
        );
        output.push_str(&line);
    }

    output.push_str(&format!("\nSummary: {} dirs, {} files, {} LOC\n",
        0, // TODO: count dirs
        results.total_files,
        results.total_loc
    ));

    output
}
