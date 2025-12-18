use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

/// Detailed file metrics (compatible with structure log format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetrics {
    pub loc: usize,
    pub fn_count: usize,
    pub pub_fn: usize,
    pub unsafe_fn: usize,
    pub imports: usize,
    pub logs_info: usize,
    pub logs_warn: usize,
    pub logs_error: usize,
    pub printlns: usize,
    pub structs: usize,
    pub enums: usize,
    pub derives: usize,
    pub tests: usize,
    pub modified: String,
}

impl FileMetrics {
    pub fn zero() -> Self {
        Self {
            loc: 0,
            fn_count: 0,
            pub_fn: 0,
            unsafe_fn: 0,
            imports: 0,
            logs_info: 0,
            logs_warn: 0,
            logs_error: 0,
            printlns: 0,
            structs: 0,
            enums: 0,
            derives: 0,
            tests: 0,
            modified: String::new(),
        }
    }
}

/// CLI Dashboard row - JSONL format with exact field names
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardRow {
    pub id: usize,
    pub n: usize,
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "relPath")]
    pub rel_path: String,
    pub link: String,
    pub progress: String,
    pub visual: String,
    pub totals: Totals,
    pub diagnostics: Vec<String>,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<FileMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Totals {
    pub crit: usize,
    pub block: usize,
    pub warn: usize,
}

impl Totals {
    pub fn zero() -> Self {
        Self { crit: 0, block: 0, warn: 0 }
    }

    pub fn new(crit: usize, block: usize, warn: usize) -> Self {
        Self { crit, block, warn }
    }
}

/// Dashboard builder with stateful counter and alias tracking
pub struct DashboardBuilder {
    total: usize,
    next_id: usize,
    alias_set: HashMap<String, bool>,
}

impl DashboardBuilder {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            next_id: 1,
            alias_set: HashMap::new(),
        }
    }

    pub fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Compute short hash (first 4 hex chars of SHA256)
    fn compute_short_hash(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result).chars().take(4).collect()
    }

    /// Create short alias from basename (max 20 chars, sanitized)
    fn short_alias(basename: &str) -> String {
        let sanitized: String = basename
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '_' || c == '.' || c == '-' { c } else { '_' })
            .collect();

        if sanitized.len() > 20 {
            sanitized.chars().take(20).collect()
        } else {
            sanitized
        }
    }

    /// Make visual progress bar (default 11 chars with '/')
    fn make_visual_bar(index: usize, total: usize, length: usize) -> String {
        if total == 0 {
            return " ".repeat(length);
        }

        let ratio = index as f64 / total as f64;
        let filled = (ratio * length as f64).round() as usize;
        let filled = filled.min(length);

        let bar = "/".repeat(filled);
        format!("{:width$}", bar, width = length)
    }

    /// Build a single dashboard row
    pub fn build_row(
        &mut self,
        rel_path: impl AsRef<Path>,
        full_path: Option<PathBuf>,
        section: Option<String>,
        totals: Totals,
    ) -> DashboardRow {
        let id = self.next_id();
        let n = id;

        let rel_path_str = rel_path.as_ref().to_string_lossy().to_string();
        let basename = rel_path.as_ref()
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let seed = full_path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| rel_path_str.clone());

        let short_hash = Self::compute_short_hash(&seed);
        let alias_base = Self::short_alias(&basename);

        let alias = if self.alias_set.contains_key(&alias_base) {
            format!("{}#{}", alias_base, short_hash)
        } else {
            self.alias_set.insert(alias_base.clone(), true);
            alias_base
        };

        let link = full_path
            .map(|p| format!("file://{}", p.to_string_lossy().replace('\\', "/")))
            .unwrap_or_else(|| rel_path_str.clone());

        let progress = if self.total > 0 {
            format!("{}/{}", id, self.total)
        } else {
            format!("{}/?", id)
        };

        let visual = Self::make_visual_bar(id, self.total, 11);

        let timestamp = chrono::Utc::now().to_rfc3339();

        DashboardRow {
            id,
            n,
            alias,
            rel_path: rel_path_str,
            link,
            progress,
            visual: visual.trim_end().to_string(),
            totals,
            diagnostics: Vec::new(),
            timestamp,
            section,
            metrics: None, // Can be populated later
        }
    }

    /// Build a row with full file metrics (for high-density logs)
    pub fn build_row_with_metrics(
        &mut self,
        rel_path: impl AsRef<Path>,
        full_path: Option<PathBuf>,
        section: Option<String>,
        totals: Totals,
        metrics: FileMetrics,
    ) -> DashboardRow {
        let mut row = self.build_row(rel_path, full_path, section, totals);
        row.metrics = Some(metrics);
        row
    }
}

impl DashboardRow {
    /// Format as plain text compact dashboard line
    /// Format: [progress][section]relPath[visual][▼crit/block/warn][▼]
    pub fn to_plain_text(&self) -> String {
        let section = self.section.as_deref().unwrap_or("Structure");
        format!(
            "[{}][{}]{}[{}][▼{}/{}/{}][▼]",
            self.progress,
            section,
            self.rel_path,
            self.visual,
            self.totals.crit,
            self.totals.block,
            self.totals.warn
        )
    }

    /// Format as high-density structure log line (compatible with existing logs)
    /// Format: · [n/total] relPath | LOC | fn | logging | structs | etc.
    pub fn to_structure_log_line(&self) -> String {
        if let Some(ref metrics) = self.metrics {
            format!(
                " · [{}/{}] {} | {} LOC | {} fn ({} pub, {} unsafe) | Imports={} | Logging: info={} warn={} error={} println={} | Structs={} Enums={} Derives={} | Errors={} | Warnings={} | Tests={} | Modified={}",
                self.n,
                self.id, // Using id as total for now
                self.rel_path,
                metrics.loc,
                metrics.fn_count,
                metrics.pub_fn,
                metrics.unsafe_fn,
                metrics.imports,
                metrics.logs_info,
                metrics.logs_warn,
                metrics.logs_error,
                metrics.printlns,
                metrics.structs,
                metrics.enums,
                metrics.derives,
                if self.totals.crit > 0 { self.totals.crit.to_string() } else { "None".to_string() },
                if self.totals.warn > 0 { self.totals.warn.to_string() } else { "None".to_string() },
                metrics.tests,
                metrics.modified
            )
        } else {
            // Fallback to compact format if no metrics
            self.to_plain_text()
        }
    }

    /// Format as baseline index JSON entry (compatible with existing JSON format)
    pub fn to_baseline_json(&self) -> Result<String, serde_json::Error> {
        if let Some(ref metrics) = self.metrics {
            let obj = serde_json::json!({
                "n": self.n,
                "total": self.id,
                "alias": self.alias,
                "relPath": self.rel_path,
                "fullPath": self.link,
                "metrics": {
                    "loc": metrics.loc,
                    "fn": metrics.fn_count,
                    "pubFn": metrics.pub_fn,
                    "unsafeFn": metrics.unsafe_fn,
                    "imports": metrics.imports,
                    "logs_info": metrics.logs_info,
                    "logs_warn": metrics.logs_warn,
                    "logs_error": metrics.logs_error,
                    "printlns": metrics.printlns,
                    "structs": metrics.structs,
                    "enums": metrics.enums,
                    "derives": metrics.derives,
                    "tests": metrics.tests,
                    "modified": metrics.modified
                }
            });
            serde_json::to_string(&obj)
        } else {
            // Fall back to JSONL format
            self.to_jsonl()
        }
    }

    /// Format as JSONL (single line JSON) - standard dashboard format
    pub fn to_jsonl(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// Build dashboard rows from a list of paths (deterministic ordering)
pub fn build_dashboard_from_paths(
    rel_paths: &[PathBuf],
    full_paths: Option<&[PathBuf]>,
    section: Option<String>,
) -> Vec<DashboardRow> {
    // Sort by relPath for deterministic ordering
    let mut pairs: Vec<(PathBuf, Option<PathBuf>)> = rel_paths
        .iter()
        .enumerate()
        .map(|(i, rel)| {
            let full = full_paths.and_then(|fps| fps.get(i).cloned());
            (rel.clone(), full)
        })
        .collect();

    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    let mut builder = DashboardBuilder::new(pairs.len());
    let mut rows = Vec::new();

    for (rel, full) in pairs {
        let row = builder.build_row(rel, full, section.clone(), Totals::zero());
        rows.push(row);
    }

    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visual_bar() {
        let bar = DashboardBuilder::make_visual_bar(2, 10, 11);
        assert_eq!(bar.trim_end().len(), 2); // 2/10 * 11 ≈ 2 slashes
    }

    #[test]
    fn test_short_alias() {
        let alias = DashboardBuilder::short_alias("very_long_filename_that_exceeds_twenty_characters.rs");
        assert_eq!(alias.len(), 20);
    }

    #[test]
    fn test_dashboard_row_plain_text() {
        let row = DashboardRow {
            id: 2,
            n: 2,
            alias: "build.rs".to_string(),
            rel_path: "bindings\\build.rs".to_string(),
            link: "file:///C:/proj/bindings/build.rs".to_string(),
            progress: "2/130".to_string(),
            visual: "///////////".to_string(),
            totals: Totals { crit: 0, block: 0, warn: 1 },
            diagnostics: Vec::new(),
            timestamp: "2025-12-18T10:00:00Z".to_string(),
            section: Some("Structure".to_string()),
        };

        let plain = row.to_plain_text();
        assert_eq!(plain, "[2/130][Structure]bindings\\build.rs[///////////][▼0/0/1][▼]");
    }

    #[test]
    fn test_dashboard_builder() {
        let mut builder = DashboardBuilder::new(3);

        let row1 = builder.build_row(
            PathBuf::from("src/lib.rs"),
            None,
            Some("Structure".to_string()),
            Totals::zero()
        );

        assert_eq!(row1.id, 1);
        assert_eq!(row1.progress, "1/3");
        assert!(row1.alias.len() <= 20);
    }

    #[test]
    fn test_alias_collision_handling() {
        let mut builder = DashboardBuilder::new(2);

        let row1 = builder.build_row(
            PathBuf::from("src/test.rs"),
            Some(PathBuf::from("C:\\proj\\src\\test.rs")),
            None,
            Totals::zero()
        );

        let row2 = builder.build_row(
            PathBuf::from("tests/test.rs"),
            Some(PathBuf::from("C:\\proj\\tests\\test.rs")),
            None,
            Totals::zero()
        );

        assert_eq!(row1.alias, "test.rs");
        assert!(row2.alias.starts_with("test.rs#"));
    }
}
