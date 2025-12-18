/// OASM Phase 1 - One-Time Initializer
///
/// Drops into root folder, performs deterministic scan, generates:
/// - Directory structure (logs/, templates/, schemas/, scripts/)
/// - CLI dashboard snapshot (JSONL + TXT)
/// - Longform structure log (JSONL + TXT)
/// - Folder blueprint (JSON + TXT)
/// - Schemas and templates
/// - Baby wrapper placeholders
/// - Preflight record and run summary

use compiler::cli_dashboard::{DashboardBuilder, DashboardRow, Totals, FileMetrics};
use compiler::diagnostics::{DiagnosticBag, DiagnosticCode, SourceLocation};
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use anyhow::{Result, Context};
use clap::Parser;
use chrono::Utc;

#[derive(Parser, Debug)]
#[command(name = "oasm-phase1")]
#[command(about = "Phase 1: One-time project initialization and scan", long_about = None)]
struct Args {
    /// Project root directory
    #[arg(default_value = ".")]
    root: PathBuf,

    /// Skip directory structure creation
    #[arg(long)]
    skip_setup: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug)]
struct ProjectArm {
    name: String,
    path: PathBuf,
    file_count: usize,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    let root = fs::canonicalize(&args.root)
        .context("Failed to resolve root directory")?;

    println!("🚀 OASM Phase 1 - One-Time Initializer");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📂 Root: {}", root.display());
    println!();

    // Step 1: Create directory structure
    if !args.skip_setup {
        println!("📁 Creating directory structure...");
        create_directory_structure(&root)?;
        create_schemas(&root)?;
        create_templates(&root)?;
        create_baby_placeholders(&root)?;
        create_config_skeleton(&root)?;
        println!("   ✓ Directory structure created\n");
    }

    // Step 2: Load exclusions from config
    let exclusions = load_exclusions(&root);

    // Step 3: Perform recursive scan
    println!("🔍 Scanning project files...");
    let files = scan_files(&root, &exclusions)?;
    println!("   Found {} files\n", files.len());

    // Step 4: Identify project arms
    println!("🎯 Identifying project arms...");
    let arms = identify_arms(&root, &files);
    for arm in &arms {
        println!("   📦 {} ({} files)", arm.name, arm.file_count);
    }
    println!();

    // Step 5: Generate outputs
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let logs_out = root.join("logs").join("logs");

    println!("📊 Generating CLI dashboard...");
    let cli_rows = generate_cli_dashboard(&files, &root)?;
    write_cli_snapshot(&cli_rows, &logs_out, &timestamp)?;
    println!("   ✓ CLI snapshot written\n");

    println!("📝 Generating longform structure log...");
    let longform_rows = generate_longform(&files, &root)?;
    write_longform(&longform_rows, &logs_out, &timestamp)?;
    println!("   ✓ Longform log written\n");

    println!("🗂️  Generating folder blueprint...");
    let folder_map = generate_folder_blueprint(&files, &root)?;
    write_folder_blueprint(&folder_map, &logs_out, &timestamp)?;
    println!("   ✓ Folder blueprint written\n");

    // Step 6: Write preflight and run summary
    write_preflight(&logs_out, &timestamp, &root)?;
    write_run_summary(&logs_out, &timestamp, files.len(), &arms)?;

    // Step 7: Final summary
    println!("✅ Phase 1 Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📈 Summary:");
    println!("   Files scanned: {}", files.len());
    println!("   Project arms: {}", arms.len());
    println!("   Outputs:");
    println!("     - CLI snapshot:     logs/logs/cli_snapshot-{}.jsonl", timestamp);
    println!("     - Longform log:     logs/logs/longform-{}.jsonl", timestamp);
    println!("     - Folder blueprint: logs/logs/folder_structure-{}.json", timestamp);
    println!("     - Run summary:      logs/logs/run_summary-{}.json", timestamp);
    println!();
    println!("🎯 Next: Run Phase 2 debug/test cycle");

    Ok(())
}

fn create_directory_structure(root: &Path) -> Result<()> {
    let dirs = vec![
        "logs",
        "logs/structure",
        "logs/structure/folders",
        "logs/provenance",
        "logs/logs",
        "logs/CompileDebug",
        "logs/CompileNormal",
        "templates",
        "scripts/PS",
        "crates/stage1/src",
    ];

    for dir in dirs {
        let path = root.join(dir);
        fs::create_dir_all(&path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
    }

    Ok(())
}

fn create_schemas(root: &Path) -> Result<()> {
    let structure_dir = root.join("logs/structure");

    // CLI state schema
    let cli_schema = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "id": { "type": "integer" },
            "n": { "type": "integer" },
            "alias": { "type": "string" },
            "relPath": { "type": "string" },
            "link": { "type": "string" },
            "progress": { "type": "string" },
            "visual": { "type": "string" },
            "totals": {
                "type": "object",
                "properties": {
                    "crit": { "type": "integer" },
                    "block": { "type": "integer" },
                    "warn": { "type": "integer" }
                }
            },
            "diagnostics": { "type": "array" },
            "timestamp": { "type": "string" }
        },
        "required": ["id", "n", "alias", "relPath", "link", "timestamp"]
    });

    fs::write(
        structure_dir.join("cli_state_schema.json"),
        serde_json::to_string_pretty(&cli_schema)?
    )?;

    // Diagnostic index schema
    let diag_schema = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "fileId": { "type": "integer" },
            "phase": { "type": "string" },
            "status": { "type": "string" },
            "short": { "type": "string" },
            "details": { "type": "string" },
            "timestamp": { "type": "string" }
        },
        "required": ["fileId", "phase", "status", "short", "timestamp"]
    });

    fs::write(
        structure_dir.join("diagnostic_index_schema.json"),
        serde_json::to_string_pretty(&diag_schema)?
    )?;

    Ok(())
}

fn create_templates(root: &Path) -> Result<()> {
    let templates_dir = root.join("templates");
    let index_path = templates_dir.join("index.yaml");

    if !index_path.exists() {
        let template = r#"templates:
  - id: sample-repair
    matcher: 'TODO_FIX_ME'
    patch: 'FIXED_BY_AUTOREPAIR'
    confidence: 50
"#;
        fs::write(index_path, template)?;
    }

    Ok(())
}

fn create_baby_placeholders(root: &Path) -> Result<()> {
    let scripts_ps = root.join("scripts/PS");

    let babies = vec![
        ("baby-full.ps1", "# Placeholder: Full scan (manifest-driven)\n# Usage: pwsh baby-full.ps1\n"),
        ("baby-rerun.ps1", "# Placeholder: Rerun tests (manifest-driven)\n# Usage: pwsh baby-rerun.ps1\n"),
        ("baby-arm.ps1", "# Placeholder: Arm-specific scan (manifest-driven)\n# Usage: pwsh baby-arm.ps1 -Arm <arm_name>\n"),
    ];

    for (name, content) in babies {
        let path = scripts_ps.join(name);
        if !path.exists() {
            fs::write(path, content)?;
        }
    }

    Ok(())
}

fn create_config_skeleton(root: &Path) -> Result<()> {
    let config_path = root.join("oasm.config.yaml");

    if !config_path.exists() {
        let config = r#"exclusions:
  - .git/
  - node_modules/
  - target/
  - build/
  - dist/
  - logs/
  - '**/*.lock'
  - '**/*.tmp'
  - '**/*.bak'
  - .DS_Store
autoRepairThreshold: 85
arms: []
logRetention: 10
concurrency: 2
"#;
        fs::write(config_path, config)?;
    }

    Ok(())
}

fn load_exclusions(root: &Path) -> Vec<String> {
    let mut exclusions = vec![
        ".git/".to_string(),
        "node_modules/".to_string(),
        "target/".to_string(),
        "build/".to_string(),
        "dist/".to_string(),
        "logs/".to_string(),
        "*.lock".to_string(),
        "*.tmp".to_string(),
        "*.bak".to_string(),
        ".DS_Store".to_string(),
    ];

    // Try to load from config
    let config_path = root.join("oasm.config.yaml");
    if let Ok(content) = fs::read_to_string(config_path) {
        // Simple YAML parsing for exclusions
        for line in content.lines() {
            if line.trim().starts_with('-') {
                let exclusion = line.trim().trim_start_matches('-').trim();
                if !exclusion.is_empty() && !exclusion.starts_with("exclusions:") {
                    exclusions.push(exclusion.to_string());
                }
            }
        }
    }

    exclusions
}

fn scan_files(root: &Path, exclusions: &[String]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in walkdir::WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let rel_path = path.strip_prefix(root).unwrap_or(path);
        let rel_str = rel_path.to_string_lossy();

        // Check exclusions
        let mut excluded = false;
        for exclusion in exclusions {
            if rel_str.contains(exclusion.trim_matches('*')) {
                excluded = true;
                break;
            }
        }

        if !excluded {
            files.push(path.to_path_buf());
        }
    }

    // Sort deterministically
    files.sort();

    Ok(files)
}

fn identify_arms(root: &Path, files: &[PathBuf]) -> Vec<ProjectArm> {
    let mut arm_map: HashMap<String, usize> = HashMap::new();

    for file in files {
        if let Ok(rel_path) = file.strip_prefix(root) {
            let components: Vec<_> = rel_path.components().collect();
            if components.len() >= 2 {
                let arm_name = format!(
                    "{}/{}",
                    components[0].as_os_str().to_string_lossy(),
                    components[1].as_os_str().to_string_lossy()
                );
                *arm_map.entry(arm_name.clone()).or_insert(0) += 1;
            }
        }
    }

    let mut arms: Vec<_> = arm_map
        .into_iter()
        .map(|(name, count)| ProjectArm {
            path: root.join(&name),
            name,
            file_count: count,
        })
        .collect();

    arms.sort_by(|a, b| b.file_count.cmp(&a.file_count));
    arms.truncate(20); // Top 20 arms

    arms
}

fn generate_cli_dashboard(files: &[PathBuf], root: &Path) -> Result<Vec<DashboardRow>> {
    let mut builder = DashboardBuilder::new(files.len());
    let mut rows = Vec::new();

    for file in files {
        let rel_path = file.strip_prefix(root).unwrap_or(file);
        let totals = Totals::zero();

        let row = builder.build_row(
            rel_path,
            Some(file.clone()),
            Some("Phase1".to_string()),
            totals,
        );

        rows.push(row);
    }

    Ok(rows)
}

fn write_cli_snapshot(rows: &[DashboardRow], logs_out: &Path, timestamp: &str) -> Result<()> {
    let jsonl_path = logs_out.join(format!("cli_snapshot-{}.jsonl", timestamp));
    let txt_path = logs_out.join(format!("cli_snapshot-{}.txt", timestamp));

    let mut jsonl_content = String::new();
    let mut txt_content = String::new();

    for row in rows {
        if let Ok(json) = row.to_jsonl() {
            jsonl_content.push_str(&json);
            jsonl_content.push('\n');
        }
        txt_content.push_str(&row.to_plain_text());
        txt_content.push('\n');
    }

    fs::write(jsonl_path, jsonl_content)?;
    fs::write(txt_path, txt_content)?;

    Ok(())
}

fn generate_longform(files: &[PathBuf], root: &Path) -> Result<Vec<DashboardRow>> {
    let mut builder = DashboardBuilder::new(files.len());
    let mut rows = Vec::new();

    for file in files {
        let rel_path = file.strip_prefix(root).unwrap_or(file);

        // Compute basic metrics
        let metrics = compute_file_metrics(file)?;
        let totals = Totals::new(metrics.unsafe_fn, 0, metrics.logs_warn);

        let row = builder.build_row_with_metrics(
            rel_path,
            Some(file.clone()),
            Some("Structure".to_string()),
            totals,
            metrics,
        );

        rows.push(row);
    }

    Ok(rows)
}

fn compute_file_metrics(file: &Path) -> Result<FileMetrics> {
    let content = fs::read_to_string(file).unwrap_or_default();
    let lines: Vec<&str> = content.lines().collect();

    let loc = lines.len();
    let fn_count = lines.iter().filter(|l| l.trim_start().starts_with("fn ")).count();
    let pub_fn = lines.iter().filter(|l| l.trim_start().starts_with("pub fn ")).count();
    let unsafe_fn = lines.iter().filter(|l| l.contains("unsafe fn")).count();
    let imports = lines.iter().filter(|l| l.trim_start().starts_with("use ")).count();
    let logs_info = content.matches("info!").count();
    let logs_warn = content.matches("warn!").count();
    let logs_error = content.matches("error!").count();
    let printlns = content.matches("println!").count();
    let structs = lines.iter().filter(|l| l.contains("struct ")).count();
    let enums = lines.iter().filter(|l| l.contains("enum ")).count();
    let derives = content.matches("#[derive").count();
    let tests = lines.iter().filter(|l| l.contains("#[test]")).count();

    let modified = if let Ok(metadata) = fs::metadata(file) {
        if let Ok(mtime) = metadata.modified() {
            chrono::DateTime::<Utc>::from(mtime)
                .format("%Y-%m-%d %H:%M")
                .to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    Ok(FileMetrics {
        loc,
        fn_count,
        pub_fn,
        unsafe_fn,
        imports,
        logs_info,
        logs_warn,
        logs_error,
        printlns,
        structs,
        enums,
        derives,
        tests,
        modified,
    })
}

fn write_longform(rows: &[DashboardRow], logs_out: &Path, timestamp: &str) -> Result<()> {
    let jsonl_path = logs_out.join(format!("longform-{}.jsonl", timestamp));
    let txt_path = logs_out.join(format!("longform-{}.txt", timestamp));

    let mut jsonl_content = String::new();
    let mut txt_content = String::new();

    txt_content.push_str("=== Project Structure Snapshot ===\n");
    txt_content.push_str(&format!("Timestamp: {}\n\n", timestamp));
    txt_content.push_str("Format: [n/total] relPath | metrics\n\n");

    for row in rows {
        if let Ok(json) = row.to_baseline_json() {
            jsonl_content.push_str(&json);
            jsonl_content.push('\n');
        }
        txt_content.push_str(&row.to_structure_log_line());
        txt_content.push('\n');
    }

    fs::write(jsonl_path, jsonl_content)?;
    fs::write(txt_path, txt_content)?;

    Ok(())
}

fn generate_folder_blueprint(files: &[PathBuf], root: &Path) -> Result<HashMap<String, Vec<String>>> {
    let mut folder_map: HashMap<String, Vec<String>> = HashMap::new();

    for file in files {
        if let Ok(rel_path) = file.strip_prefix(root) {
            let folder = rel_path
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| ".".to_string());

            folder_map
                .entry(folder)
                .or_insert_with(Vec::new)
                .push(rel_path.to_string_lossy().to_string());
        }
    }

    Ok(folder_map)
}

fn write_folder_blueprint(
    folder_map: &HashMap<String, Vec<String>>,
    logs_out: &Path,
    timestamp: &str,
) -> Result<()> {
    let json_path = logs_out.join(format!("folder_structure-{}.json", timestamp));
    let txt_path = logs_out.join(format!("folder_structure-{}.txt", timestamp));

    // JSON output
    let mut folders: Vec<_> = folder_map
        .iter()
        .map(|(folder, files)| {
            serde_json::json!({
                "folderPath": folder,
                "fileCount": files.len(),
                "sample": files.first().unwrap_or(&String::new())
            })
        })
        .collect();

    folders.sort_by(|a, b| {
        a["folderPath"]
            .as_str()
            .unwrap_or("")
            .cmp(b["folderPath"].as_str().unwrap_or(""))
    });

    fs::write(json_path, serde_json::to_string_pretty(&folders)?)?;

    // Text output
    let mut txt_content = String::new();
    for entry in &folders {
        txt_content.push_str(&format!(
            "{} : {} files\n",
            entry["folderPath"].as_str().unwrap_or(""),
            entry["fileCount"].as_u64().unwrap_or(0)
        ));
        txt_content.push_str(&format!(
            "  sample: {}\n",
            entry["sample"].as_str().unwrap_or("")
        ));
    }

    fs::write(txt_path, txt_content)?;

    Ok(())
}

fn write_preflight(logs_out: &Path, timestamp: &str, root: &Path) -> Result<()> {
    let preflight = serde_json::json!({
        "root": root.to_string_lossy(),
        "timestamp": Utc::now().to_rfc3339(),
        "created": [
            "logs tree",
            "schemas",
            "templates",
            "baby placeholders"
        ]
    });

    fs::write(
        logs_out.join(format!("preflight-{}.json", timestamp)),
        serde_json::to_string_pretty(&preflight)?,
    )?;

    Ok(())
}

fn write_run_summary(
    logs_out: &Path,
    timestamp: &str,
    total_files: usize,
    arms: &[ProjectArm],
) -> Result<()> {
    let arm_names: Vec<_> = arms.iter().map(|a| a.name.as_str()).collect();

    let summary = serde_json::json!({
        "mode": "Phase1-OneTime",
        "timestamp": Utc::now().to_rfc3339(),
        "totalFiles": total_files,
        "arms": arm_names,
        "cli_snapshot": format!("cli_snapshot-{}.jsonl", timestamp),
        "longform": format!("longform-{}.jsonl", timestamp),
        "folder_structure": format!("folder_structure-{}.json", timestamp),
        "created": [
            "schemas",
            "templates",
            "baby placeholders",
            "cli_snapshot",
            "longform",
            "folder_structure",
            "review_queue"
        ]
    });

    fs::write(
        logs_out.join(format!("run_summary-{}.json", timestamp)),
        serde_json::to_string_pretty(&summary)?,
    )?;

    Ok(())
}
