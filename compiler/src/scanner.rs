/// Universal Codebase Scanner
/// Pre-compile diagnostic and parse tool for any project root
///
/// Generates:
/// - structure_{timestamp}.log (tree view with metrics)
/// - baseline_index_{timestamp}.json (structured data)
/// - cli_state_{timestamp}.json (compilation status)

use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetrics {
    pub n: usize,
    pub total: usize,
    pub stage: String,
    pub alias: String,
    pub rel_path: String,
    pub progress: String,
    pub crit: usize,
    pub block: usize,
    pub warn: usize,
    pub compile_root: Option<String>,

    // Extended metrics
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

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingMetrics {
    pub info: usize,
    pub warn: usize,
    pub error: usize,
    pub println: usize,
}

#[derive(Debug, Serialize)]
pub struct StructureLog {
    pub root: String,
    pub timestamp: String,
    pub total_files: usize,
    pub total_dirs: usize,
    pub total_loc: usize,
    pub files: Vec<FileMetrics>,
}

pub struct Scanner {
    root: PathBuf,
    patterns: Vec<String>,
}

impl Scanner {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
            patterns: vec![
                "**/*.rs".to_string(),
                "**/*.toml".to_string(),
                "**/*.py".to_string(),
                "**/*.c".to_string(),
                "**/*.cpp".to_string(),
                "**/*.h".to_string(),
                "**/*.hpp".to_string(),
                "**/*.ps1".to_string(),
                "**/*.yaml".to_string(),
                "**/*.yml".to_string(),
                "**/*.md".to_string(),
            ],
        }
    }

    /// Scan project and return structured metrics
    pub fn scan(&self) -> Result<StructureLog> {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        let mut files = Vec::new();
        let mut file_count = 0;

        // Walk directory tree
        for entry in walkdir::WalkDir::new(&self.root)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Some(metrics) = self.analyze_file(&entry.path(), file_count) {
                    files.push(metrics);
                    file_count += 1;
                }
            }
        }

        let total_loc: usize = files.iter().map(|f| f.loc).sum();

        Ok(StructureLog {
            root: self.root.display().to_string(),
            timestamp,
            total_files: files.len(),
            total_dirs: 0, // TODO: count directories
            total_loc,
            files,
        })
    }

    /// Analyze a single file and extract metrics
    fn analyze_file(&self, path: &Path, index: usize) -> Option<FileMetrics> {
        let ext = path.extension()?.to_str()?;

        // Check if extension matches our patterns
        let valid_exts = ["rs", "toml", "py", "c", "cpp", "h", "hpp", "ps1", "yaml", "yml", "md"];
        if !valid_exts.contains(&ext) {
            return None;
        }

        let content = fs::read_to_string(path).ok()?;
        let lines: Vec<&str> = content.lines().collect();
        let loc = lines.len();

        // Calculate metrics based on file type
        let metrics = match ext {
            "rs" => self.analyze_rust(&lines),
            "py" => self.analyze_python(&lines),
            "cpp" | "c" | "hpp" | "h" => self.analyze_c_cpp(&lines),
            "ps1" => self.analyze_powershell(&lines),
            _ => (0, 0, 0, 0, LoggingMetrics::default(), 0, 0, 0, 0),
        };

        let rel_path = path.strip_prefix(&self.root)
            .ok()?
            .to_string_lossy()
            .to_string();

        let modified = fs::metadata(path)
            .ok()?
            .modified()
            .ok()
            .and_then(|t| {
                let datetime: chrono::DateTime<chrono::Local> = t.into();
                Some(datetime.format("%Y-%m-%d %H:%M").to_string())
            })
            .unwrap_or_else(|| "unknown".to_string());

        Some(FileMetrics {
            n: index + 1,
            total: 0, // Will be filled later
            stage: "Debug".to_string(),
            alias: path.file_name()?.to_string_lossy().to_string(),
            rel_path,
            progress: "[///////////////]".to_string(),
            crit: 0,
            block: 0,
            warn: 0,
            compile_root: None,
            loc,
            fn_count: metrics.0,
            pub_fn_count: metrics.1,
            unsafe_fn_count: metrics.2,
            imports: metrics.3,
            logging: metrics.4,
            structs: metrics.5,
            enums: metrics.6,
            derives: metrics.7,
            tests: metrics.8,
            modified,
        })
    }

    fn analyze_rust(&self, lines: &[&str]) -> (usize, usize, usize, usize, LoggingMetrics, usize, usize, usize, usize) {
        let fn_count = lines.iter().filter(|l| l.trim_start().starts_with("fn ")).count();
        let pub_fn_count = lines.iter().filter(|l| l.trim_start().starts_with("pub fn ")).count();
        let unsafe_fn_count = lines.iter().filter(|l| l.contains("unsafe")).count();
        let imports = lines.iter().filter(|l| l.trim_start().starts_with("use ")).count();

        let info_logs = lines.iter().filter(|l| l.contains("info!")).count();
        let warn_logs = lines.iter().filter(|l| l.contains("warn!")).count();
        let error_logs = lines.iter().filter(|l| l.contains("error!")).count();
        let printlns = lines.iter().filter(|l| l.contains("println!")).count();

        let structs = lines.iter().filter(|l| l.trim_start().starts_with("pub struct ") || l.trim_start().starts_with("struct ")).count();
        let enums = lines.iter().filter(|l| l.trim_start().starts_with("pub enum ") || l.trim_start().starts_with("enum ")).count();
        let derives = lines.iter().filter(|l| l.contains("#[derive")).count();
        let tests = lines.iter().filter(|l| l.contains("#[test]")).count();

        (
            fn_count,
            pub_fn_count,
            unsafe_fn_count,
            imports,
            LoggingMetrics { info: info_logs, warn: warn_logs, error: error_logs, println: printlns },
            structs,
            enums,
            derives,
            tests,
        )
    }

    fn analyze_python(&self, lines: &[&str]) -> (usize, usize, usize, usize, LoggingMetrics, usize, usize, usize, usize) {
        let fn_count = lines.iter().filter(|l| l.trim_start().starts_with("def ")).count();
        let imports = lines.iter().filter(|l| {
            let trimmed = l.trim_start();
            trimmed.starts_with("import ") || trimmed.starts_with("from ")
        }).count();

        let info_logs = lines.iter().filter(|l| l.contains("logging.info") || l.contains("logger.info")).count();
        let warn_logs = lines.iter().filter(|l| l.contains("logging.warn") || l.contains("logger.warn")).count();
        let error_logs = lines.iter().filter(|l| l.contains("logging.error") || l.contains("logger.error")).count();
        let prints = lines.iter().filter(|l| l.contains("print(")).count();

        let classes = lines.iter().filter(|l| l.trim_start().starts_with("class ")).count();

        (fn_count, fn_count, 0, imports, LoggingMetrics { info: info_logs, warn: warn_logs, error: error_logs, println: prints }, classes, 0, 0, 0)
    }

    fn analyze_c_cpp(&self, lines: &[&str]) -> (usize, usize, usize, usize, LoggingMetrics, usize, usize, usize, usize) {
        // Simple heuristic for C/C++
        let fn_count = lines.iter().filter(|l| {
            let trimmed = l.trim();
            trimmed.contains("(") && trimmed.contains(")") && trimmed.contains("{") && !trimmed.starts_with("//")
        }).count();

        let includes = lines.iter().filter(|l| l.trim_start().starts_with("#include")).count();
        let printfs = lines.iter().filter(|l| l.contains("printf") || l.contains("std::cout")).count();

        (fn_count, 0, 0, includes, LoggingMetrics { info: 0, warn: 0, error: 0, println: printfs }, 0, 0, 0, 0)
    }

    fn analyze_powershell(&self, lines: &[&str]) -> (usize, usize, usize, usize, LoggingMetrics, usize, usize, usize, usize) {
        let fn_count = lines.iter().filter(|l| l.trim_start().starts_with("function ")).count();
        let imports = lines.iter().filter(|l| l.trim_start().starts_with("using ") || l.trim_start().starts_with("Import-Module")).count();

        let write_hosts = lines.iter().filter(|l| l.contains("Write-Host")).count();
        let write_warns = lines.iter().filter(|l| l.contains("Write-Warning")).count();
        let write_errors = lines.iter().filter(|l| l.contains("Write-Error")).count();

        (fn_count, 0, 0, imports, LoggingMetrics { info: write_hosts, warn: write_warns, error: write_errors, println: 0 }, 0, 0, 0, 0)
    }
}

impl Default for LoggingMetrics {
    fn default() -> Self {
        Self { info: 0, warn: 0, error: 0, println: 0 }
    }
}
