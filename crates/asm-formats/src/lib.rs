//! OASM Data Format Schemas
//!
//! Architecture:
//! - HDF5: Immutable canonical templates (baseline snapshots, deep artifacts)
//! - CBOR: Runtime binary objects (fast IPC, deterministic execution)
//! - YAML: Human-readable overlays (annotated, auto-populated)
//! - JSON: Lineage logs (audit trails, Git-friendly)
//!
//! Flow: HDF5 → CBOR/YAML → Execute → Discard → JSON Lineage

pub mod schemas;
pub mod templates;
pub mod runtime;
pub mod lineage;
pub mod converters;
pub mod domains;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Universal identifier for tracking runs, sequences, and diffs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RunId(pub Uuid);

impl RunId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_string(s: &str) -> anyhow::Result<Self> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl Default for RunId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RunId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Sequence number within a run
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Seq(pub u64);

impl Seq {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

/// Actor performing the operation (human, automation, AI)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Actor {
    Human { username: String },
    Automation { rule_id: String },
    AI { model: String, confidence: f64 },
    System,
}

/// Confidence level for automated actions
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Confidence(pub f64);

impl Confidence {
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    pub fn high() -> Self {
        Self(0.9)
    }

    pub fn medium() -> Self {
        Self(0.7)
    }

    pub fn low() -> Self {
        Self(0.5)
    }

    pub fn exceeds_threshold(&self, threshold: f64) -> bool {
        self.0 >= threshold
    }
}

/// Test execution status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestStatus {
    Planned,
    Running,
    Passed,
    Failed { reason: String },
    Skipped,
}

/// Impact metrics for changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Impact {
    pub files_changed: usize,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub functions_affected: usize,
    pub modules_affected: Vec<String>,
}

/// Common metadata for all execution units
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetadata {
    pub run_id: RunId,
    pub seq: Seq,
    pub timestamp: DateTime<Utc>,
    pub actor: Actor,
    pub tool_versions: ToolVersions,
    pub config_hash: String,
}

impl ExecutionMetadata {
    pub fn new(actor: Actor) -> Self {
        Self {
            run_id: RunId::new(),
            seq: Seq::zero(),
            timestamp: Utc::now(),
            actor,
            tool_versions: ToolVersions::current(),
            config_hash: String::new(), // Populated by caller
        }
    }
}

/// Tool version tracking for provenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolVersions {
    pub oasm_version: String,
    pub rust_version: String,
    pub llvm_version: Option<String>,
    pub python_version: Option<String>,
}

impl ToolVersions {
    pub fn current() -> Self {
        Self {
            oasm_version: env!("CARGO_PKG_VERSION").to_string(),
            rust_version: "1.70+".to_string(), // TODO: detect actual version
            llvm_version: None,
            python_version: None,
        }
    }
}

/// Format type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormatType {
    HDF5,
    CBOR,
    YAML,
    JSON,
}

/// Encoding choice (popup decision point)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Encoding {
    UTF8,
    ASCII,
}

/// Duplicate handling strategy (popup decision point)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DuplicateStrategy {
    Erase,
    EvaluateDiff,
    Ignore,
}

/// Language/runtime choice (popup decision point)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    CPlusPlus,
    Python,
    Rust,
    Go,
    JavaScript,
}

/// User decision from popup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopupDecision {
    pub timestamp: DateTime<Utc>,
    pub prompt: String,
    pub decision: String,
    pub options_presented: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_id_creation() {
        let id1 = RunId::new();
        let id2 = RunId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_seq_ordering() {
        let seq1 = Seq(1);
        let seq2 = Seq(2);
        assert!(seq1 < seq2);
        assert_eq!(seq1.next(), seq2);
    }

    #[test]
    fn test_confidence_clamping() {
        let c1 = Confidence::new(1.5);
        assert_eq!(c1.0, 1.0);

        let c2 = Confidence::new(-0.5);
        assert_eq!(c2.0, 0.0);
    }

    #[test]
    fn test_confidence_threshold() {
        let c = Confidence::new(0.85);
        assert!(c.exceeds_threshold(0.8));
        assert!(!c.exceeds_threshold(0.9));
    }
}
