use runtime_daemon::parser::{parse_manifest, to_yaml};
use runtime_daemon::validator::validate_manifest;
use runtime_daemon::commit::commit_text;
use runtime_daemon::lineage::record_event;

pub mod scanner;
pub mod diagnostics;
pub mod cli_dashboard;

use diagnostics::{DiagnosticBag, DiagnosticCode, SourceLocation};
use cli_dashboard::DashboardBuilder;
use std::path::PathBuf;

pub fn compile_manifest(path: &str) -> Result<(), String> {
    compile_manifest_with_diagnostics(path, false)
}

pub fn compile_manifest_with_diagnostics(path: &str, enable_dashboard: bool) -> Result<(), String> {
    log::info!("Compiler invoked on manifest: {}", path);

    let mut diagnostics = DiagnosticBag::new();

    // Parse manifest
    let manifest = match parse_manifest(path) {
        Ok(m) => m,
        Err(e) => {
            diagnostics.add_error(
                DiagnosticCode::E0500,
                format!("Failed to parse manifest: {}", e),
                SourceLocation::new(PathBuf::from(path), 0, 0, 0)
            );
            if enable_dashboard {
                emit_dashboard_for_path(path, &diagnostics);
            }
            diagnostics.print_all();
            return Err(format!("Parsing failed: {}", e));
        }
    };

    // Validate manifest
    let validated = match validate_manifest(&manifest) {
        Ok(v) => v,
        Err(e) => {
            diagnostics.add_error(
                DiagnosticCode::E0501,
                format!("Manifest validation failed: {}", e),
                SourceLocation::new(PathBuf::from(path), 0, 0, 0)
            );
            if enable_dashboard {
                emit_dashboard_for_path(path, &diagnostics);
            }
            diagnostics.print_all();
            return Err(format!("Validation failed: {}", e));
        }
    };

    // Convert validated manifest back to YAML
    let yaml_contents = match to_yaml(&validated) {
        Ok(y) => y,
        Err(e) => {
            diagnostics.add_error(
                DiagnosticCode::E0500,
                format!("YAML serialization failed: {}", e),
                SourceLocation::new(PathBuf::from(path), 0, 0, 0)
            );
            if enable_dashboard {
                emit_dashboard_for_path(path, &diagnostics);
            }
            diagnostics.print_all();
            return Err(format!("YAML serialization failed: {}", e));
        }
    };

    // Commit the validated YAML
    if let Err(e) = commit_text(path, &yaml_contents) {
        diagnostics.add_error(
            DiagnosticCode::E0500,
            format!("Commit failed: {}", e),
            SourceLocation::new(PathBuf::from(path), 0, 0, 0)
        );
        if enable_dashboard {
            emit_dashboard_for_path(path, &diagnostics);
        }
        diagnostics.print_all();
        return Err(format!("Commit failed: {}", e));
    }

    // Success - emit dashboard if enabled
    if enable_dashboard {
        emit_dashboard_for_path(path, &diagnostics);
    }

    record_event(&format!("Manifest compiled successfully: {}", path)).ok();

    if diagnostics.has_errors() {
        diagnostics.print_summary();
        Err("Compilation completed with errors".to_string())
    } else {
        log::info!("Compilation successful");
        Ok(())
    }
}

fn emit_dashboard_for_path(path: &str, diagnostics: &DiagnosticBag) {
    let mut builder = DashboardBuilder::new(1);
    let totals = diagnostics.to_dashboard_totals();

    let mut row = builder.build_row(
        PathBuf::from(path),
        Some(PathBuf::from(path)),
        Some("Compile".to_string()),
        totals
    );

    diagnostics.attach_to_dashboard(&mut row);

    // Print dashboard row in plain text format
    println!("{}", row.to_plain_text());

    // Print JSONL format for parsing
    if let Ok(jsonl) = row.to_jsonl() {
        eprintln!("DASHBOARD_JSONL: {}", jsonl);
    }
}
