//! Schema definitions for HDF5, CBOR, YAML, and JSON formats
//!
//! These schemas define the structure of data at each layer:
//! - HDF5: Immutable templates (canonical baseline)
//! - CBOR: Runtime execution objects (binary, deterministic)
//! - YAML: Human-readable overlays (annotated, auto-populated)
//! - JSON: Lineage logs (audit trails, Git-friendly)

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::{RunId, Seq, Actor, Confidence, Impact, ExecutionMetadata, TestStatus, PopupDecision};

/// HDF5 Template Schema (Immutable Canonical)
///
/// Stored in HDF5 datasets, these templates are never modified.
/// They represent the "gold record" baseline for tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HDF5Template {
    pub template_id: String,
    pub version: String,
    pub created: DateTime<Utc>,
    pub description: String,

    /// Template type (assembler_pass, compiler_stage, test_harness, etc.)
    pub template_type: TemplateType,

    /// Deep state snapshots (CFG, DFG, baseline conversions)
    pub artifacts: Vec<Artifact>,

    /// Immutable baseline data
    pub baseline: BaselineSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateType {
    AssemblerPass,
    CompilerStage,
    TestHarness,
    RepairTemplate,
    LintBundle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub artifact_id: String,
    pub artifact_type: ArtifactType,
    pub data_path: String, // HDF5 dataset path
    pub size_bytes: u64,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactType {
    CFG, // Control Flow Graph
    DFG, // Data Flow Graph
    SymbolTable,
    TestFixture,
    LargeDataset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineSnapshot {
    pub snapshot_id: String,
    pub timestamp: DateTime<Utc>,
    pub files: Vec<FileSnapshot>,
    pub metrics: BaselineMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSnapshot {
    pub file_path: String,
    pub file_hash: String,
    pub loc: usize,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineMetrics {
    pub total_files: usize,
    pub total_loc: usize,
    pub total_functions: usize,
    pub complexity_score: f64,
}

/// CBOR Runtime Object Schema (Binary Execution Truth)
///
/// Generated from HDF5 templates or YAML overlays.
/// Compact, deterministic binary schema for fast IPC.
/// Immutable once created for a run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CBORRuntimeObject {
    pub object_id: String,
    pub metadata: ExecutionMetadata,

    /// Command block to execute
    pub command: CommandBlock,

    /// Auto-populated fields
    pub auto_fields: AutoPopulatedFields,

    /// User decisions from popups (if any)
    pub decisions: Vec<PopupDecision>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandBlock {
    pub block_type: BlockType,
    pub parameters: Vec<Parameter>,
    pub target_files: Vec<String>,
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockType {
    LintCheck,
    RepairBlock,
    TestRunner,
    AnalysisPass,
    Converter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub key: String,
    pub value: ParameterValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoPopulatedFields {
    pub run_id: RunId,
    pub seq: Seq,
    pub timestamp: DateTime<Utc>,
    pub actor: Actor,
    pub file_path: Option<String>,
    pub rule_group: Option<String>,
    pub confidence: Option<Confidence>,
    pub tests_planned: Vec<String>,
}

/// YAML Overlay Schema (Human-Readable Annotated)
///
/// Mirrors CBOR structure but with comments and annotations.
/// Auto-populated fields for convenience.
/// Validated → converted → executed only as CBOR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YAMLOverlay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    pub metadata: ExecutionMetadata,
    pub command: CommandBlock,

    /// Auto-populated by system
    pub auto_populated: AutoPopulatedFields,

    /// Annotations for human understanding
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub field: String,
    pub explanation: String,
    pub rationale: Option<String>,
}

/// JSON Lineage Schema (Audit Trail)
///
/// Records execution outcomes, decisions, and provenance.
/// Git-friendly format for version control.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONLineage {
    pub lineage_id: String,
    pub run_id: RunId,
    pub seq: Seq,
    pub timestamp: DateTime<Utc>,
    pub actor: Actor,

    /// Task summary
    pub summary: String,
    pub intent: String,

    /// Execution details
    pub command_executed: String,
    pub outcome: ExecutionOutcome,

    /// Provenance
    pub provenance: Provenance,

    /// Impact metrics
    pub impact: Impact,

    /// Tests
    pub tests: Vec<TestRecord>,

    /// Diff reference (if applicable)
    pub diff_id: Option<String>,

    /// Git integration
    pub git_sha: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionOutcome {
    Success,
    Failed { reason: String },
    PartialSuccess { warnings: Vec<String> },
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub tool_versions: crate::ToolVersions,
    pub config_hash: String,
    pub template_id: Option<String>,
    pub parent_run_id: Option<RunId>,
    pub lineage_chain: Vec<String>,
    pub confidence: Option<Confidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRecord {
    pub test_id: String,
    pub test_name: String,
    pub status: TestStatus,
    pub duration_ms: Option<u64>,
    pub logs: Vec<String>,
}

/// Diff Snapshot Schema
///
/// Unified diff format with YAML header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffSnapshot {
    /// YAML header
    pub header: DiffHeader,

    /// Diff payload (unified format)
    pub hunks: Vec<DiffHunk>,

    /// Compression info
    pub compression: Option<CompressionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffHeader {
    pub diff_id: String,
    pub run_id: RunId,
    pub seq: Seq,
    pub timestamp: DateTime<Utc>,
    pub actor: Actor,
    pub summary: String,
    pub confidence: Confidence,
    pub intent: String,
    pub tests: Vec<String>,
    pub impact: Impact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffHunk {
    pub file_path: String,
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    pub line_type: DiffLineType,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffLineType {
    Context,   // unchanged
    Removal,   // red -
    Addition,  // green +
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionInfo {
    pub algorithm: CompressionAlgorithm,
    pub original_size: u64,
    pub compressed_size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    Gzip,
    Zstd,
    None,
}

/// Session Index (ordered diffs for a run)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionIndex {
    pub run_id: RunId,
    pub started: DateTime<Utc>,
    pub ended: Option<DateTime<Utc>>,
    pub diffs: Vec<DiffReference>,
    pub totals: SessionTotals,
    pub git_shas: Vec<String>,
    pub provenance_links: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffReference {
    pub seq: Seq,
    pub diff_id: String,
    pub timestamp: DateTime<Utc>,
    pub summary: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTotals {
    pub total_diffs: usize,
    pub files_changed: usize,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub tests_run: usize,
    pub tests_passed: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hdf5_template_serialization() {
        let template = HDF5Template {
            template_id: "test_template_001".to_string(),
            version: "1.0.0".to_string(),
            created: Utc::now(),
            description: "Test template".to_string(),
            template_type: TemplateType::AssemblerPass,
            artifacts: vec![],
            baseline: BaselineSnapshot {
                snapshot_id: "snap_001".to_string(),
                timestamp: Utc::now(),
                files: vec![],
                metrics: BaselineMetrics {
                    total_files: 0,
                    total_loc: 0,
                    total_functions: 0,
                    complexity_score: 0.0,
                },
            },
        };

        let json = serde_json::to_string(&template).unwrap();
        assert!(json.contains("test_template_001"));
    }

    #[test]
    fn test_cbor_runtime_object() {
        let obj = CBORRuntimeObject {
            object_id: "obj_001".to_string(),
            metadata: ExecutionMetadata::new(Actor::System),
            command: CommandBlock {
                block_type: BlockType::LintCheck,
                parameters: vec![],
                target_files: vec![],
                rules: vec![],
            },
            auto_fields: AutoPopulatedFields {
                run_id: RunId::new(),
                seq: Seq::zero(),
                timestamp: Utc::now(),
                actor: Actor::System,
                file_path: None,
                rule_group: None,
                confidence: None,
                tests_planned: vec![],
            },
            decisions: vec![],
        };

        let cbor = serde_cbor::to_vec(&obj).unwrap();
        assert!(!cbor.is_empty());
    }
}
