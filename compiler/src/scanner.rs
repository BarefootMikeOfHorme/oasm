use std::collections::HashMap;
use std::path::PathBuf;
use chrono;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use crate::cli_dashboard::{DashboardBuilder, DashboardRow, Totals};

#[derive(Debug, Clone)]
pub struct Scanner {
    pub root_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureLog {
    pub root: String,
    pub timestamp: String,
    pub total_files: usize,
    pub total_lines: usize,
    pub total_loc: usize,
    pub files: Vec<FileInfo>,
    pub file_details: HashMap<String, FileMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub n: usize,
    pub rel_path: String,
    pub alias: String,
    pub loc: usize,
    pub fn_count: usize,
    pub pub_fn_count: usize,
    pub unsafe_fn_count: usize,
    pub imports: usize,
    pub logging: LoggingMetrics,
    pub structs: usize,
    pub enums: usize,
    pub derives: usize,
    pub tests: usize,
    pub modified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingMetrics {
    pub info: usize,
    pub warn: usize,
    pub error: usize,
    pub println: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetrics {
    pub lines: usize,
    pub functions: usize,
    pub structs: usize,
}

impl Scanner {
    pub fn new(root_path: impl AsRef<std::path::Path>) -> Self {
        Self {
            root_path: root_path.as_ref().to_string_lossy().to_string()
        }
    }

    pub fn scan(&self) -> Result<StructureLog> {
        // TODO: Implement actual scanning logic
        // For now, return a valid but empty structure
        Ok(StructureLog {
            root: self.root_path.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            total_files: 0,
            total_lines: 0,
            total_loc: 0,
            files: Vec::new(),
            file_details: HashMap::new(),
        })
    }

    /// Scan and emit dashboard format (JSONL + plain text)
    pub fn scan_with_dashboard(&self) -> Result<Vec<DashboardRow>> {
        let structure_log = self.scan()?;

        // Convert FileInfo entries to dashboard rows
        let mut builder = DashboardBuilder::new(structure_log.files.len());
        let mut rows = Vec::new();

        for file_info in &structure_log.files {
            let rel_path = PathBuf::from(&file_info.rel_path);

            // Calculate totals from file metrics
            let totals = Totals::new(
                file_info.unsafe_fn_count,  // crit: unsafe functions
                0,                            // block: none by default
                file_info.logging.warn       // warn: warning log calls
            );

            let row = builder.build_row(
                rel_path,
                None,
                Some("Structure".to_string()),
                totals
            );

            rows.push(row);
        }

        Ok(rows)
    }
}

pub fn scan_manifest(path: &str) -> Result<()> {
    println!("Scanning manifest at {}", path);
    Ok(())
}
