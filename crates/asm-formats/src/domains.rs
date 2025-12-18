//! Domain-Specific Schemas
//!
//! Each domain (folder structure, logging, shells, code modules, binary artifacts)
//! has its own HDF5→CBOR→YAML→JSON pipeline with domain-specific schemas.
//!
//! INTEGRATION HIERARCHY:
//! 1. ASM (OASM Core) - Base layer, format schemas, execution engine
//! 2. Shells (PowerShell/Bash) - Orchestration layer, command blocks, automation
//! 3. Rust & Python Modules - Extension layer, domain-specific functionality
//!
//! Flow: ASM provides schemas → Shells orchestrate tasks → Rust/Py extend capabilities
//!
//! Universal pattern: Immutable HDF5 generates working copies for tasks.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::{RunId, Seq};
use std::path::PathBuf;

/// Universal artifact stored in HDF5 (any file type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmutableArtifact {
    pub artifact_id: String,
    pub artifact_type: ArtifactType,
    pub version: String,
    pub created: DateTime<Utc>,

    /// Original file path or identifier
    pub source_path: String,

    /// HDF5 dataset path where artifact is stored
    pub hdf5_path: String,

    /// Metadata about the artifact
    pub metadata: ArtifactMetadata,

    /// Checksum for integrity verification
    pub checksum: String,

    /// Size in bytes
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactType {
    // Binary artifacts
    Image { format: ImageFormat },
    Program { platform: String, arch: String },
    Object3D { format: String }, // STL, OBJ, FBX, etc.

    // Document artifacts
    Document { format: DocumentFormat },

    // Code artifacts
    SourceCode { language: String },
    Library { language: String, linkage: String },

    // Data artifacts
    Dataset { format: DataFormat },

    // Configuration artifacts
    Config { format: ConfigFormat },

    // Template artifacts
    Template { template_type: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    PNG,
    JPG,
    SVG,
    TIFF,
    BMP,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentFormat {
    PDF,
    DOCX,
    Markdown,
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataFormat {
    CSV,
    Parquet,
    JSON,
    Binary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigFormat {
    YAML,
    TOML,
    JSON,
    INI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactMetadata {
    pub author: Option<String>,
    pub description: String,
    pub tags: Vec<String>,
    pub parent_artifact_id: Option<String>,
    pub custom_fields: std::collections::HashMap<String, String>,
}

/// Working copy generated from immutable artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingCopy {
    pub copy_id: String,
    pub run_id: RunId,
    pub seq: Seq,
    pub source_artifact_id: String,
    pub created: DateTime<Utc>,

    /// Working directory where copy resides
    pub working_path: PathBuf,

    /// Modifications applied to this copy
    pub modifications: Vec<Modification>,

    /// Status of working copy
    pub status: WorkingCopyStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modification {
    pub timestamp: DateTime<Utc>,
    pub operation: String,
    pub parameters: Vec<String>,
    pub actor: crate::Actor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkingCopyStatus {
    Active,
    Completed { outcome: String },
    Failed { reason: String },
    Discarded,
}

//
// DOMAIN 1: Folder Structure
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderStructureDomain {
    pub domain_id: String,
    pub root_path: PathBuf,
    pub snapshot: FolderSnapshot,
    pub hdf5_reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderSnapshot {
    pub snapshot_id: String,
    pub timestamp: DateTime<Utc>,
    pub folders: Vec<FolderEntry>,
    pub files: Vec<FileEntry>,
    pub total_size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderEntry {
    pub path: PathBuf,
    pub file_count: usize,
    pub subfolder_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub modified: DateTime<Utc>,
    pub checksum: String,
}

//
// DOMAIN 2: Logging
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingDomain {
    pub domain_id: String,
    pub log_type: LogType,
    pub entries: Vec<LogEntry>,
    pub hdf5_reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogType {
    ProgramOutput,
    ShellOutput,
    SystemLog,
    DiagnosticLog,
    AuditLog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub source: String,
    pub message: String,
    pub context: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

//
// DOMAIN 3: Shell Modules
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellModuleDomain {
    pub domain_id: String,
    pub shell_type: ShellType,
    pub script_path: PathBuf,
    pub variables: std::collections::HashMap<String, String>,
    pub hdf5_reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShellType {
    PowerShell,
    Bash,
    Zsh,
    Fish,
    Cmd,
}

//
// DOMAIN 4: Python Modules
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonModuleDomain {
    pub domain_id: String,
    pub module_name: String,
    pub module_path: PathBuf,
    pub dependencies: Vec<PythonDependency>,
    pub config: PythonConfig,
    pub hdf5_reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonDependency {
    pub package: String,
    pub version: String,
    pub extras: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonConfig {
    pub python_version: String,
    pub virtual_env: Option<PathBuf>,
    pub environment_vars: std::collections::HashMap<String, String>,
}

//
// DOMAIN 5: Rust Modules
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustModuleDomain {
    pub domain_id: String,
    pub crate_name: String,
    pub crate_path: PathBuf,
    pub dependencies: Vec<RustDependency>,
    pub features: Vec<String>,
    pub target: RustTarget,
    pub hdf5_reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustDependency {
    pub name: String,
    pub version: String,
    pub features: Vec<String>,
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustTarget {
    pub triple: String,
    pub profile: BuildProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildProfile {
    Debug,
    Release,
    Custom { name: String },
}

//
// Copy-on-Work Manager
//

pub struct CopyOnWorkManager {
    immutable_store_path: PathBuf,
    working_dir: PathBuf,
}

impl CopyOnWorkManager {
    pub fn new(immutable_store: PathBuf, working_dir: PathBuf) -> Self {
        Self {
            immutable_store_path: immutable_store,
            working_dir,
        }
    }

    /// Create working copy from immutable artifact
    pub fn create_working_copy(
        &self,
        artifact: &ImmutableArtifact,
        run_id: RunId,
        seq: Seq,
    ) -> anyhow::Result<WorkingCopy> {
        let copy_id = format!("copy_{}_{}", run_id, seq.0);
        let working_path = self.working_dir.join(&copy_id);

        // TODO: Implement actual copy from HDF5
        // For now, create placeholder directory
        std::fs::create_dir_all(&working_path)?;

        Ok(WorkingCopy {
            copy_id,
            run_id,
            seq,
            source_artifact_id: artifact.artifact_id.clone(),
            created: Utc::now(),
            working_path,
            modifications: Vec::new(),
            status: WorkingCopyStatus::Active,
        })
    }

    /// Record modification to working copy
    pub fn record_modification(
        &self,
        copy: &mut WorkingCopy,
        operation: String,
        parameters: Vec<String>,
        actor: crate::Actor,
    ) {
        copy.modifications.push(Modification {
            timestamp: Utc::now(),
            operation,
            parameters,
            actor,
        });
    }

    /// Commit working copy as new immutable version
    pub fn commit_as_immutable(
        &self,
        copy: &WorkingCopy,
        new_version: String,
    ) -> anyhow::Result<ImmutableArtifact> {
        // TODO: Implement actual commit to HDF5
        // For now, return placeholder

        Ok(ImmutableArtifact {
            artifact_id: format!("{}_v{}", copy.source_artifact_id, new_version),
            artifact_type: ArtifactType::Template {
                template_type: "modified".to_string(),
            },
            version: new_version,
            created: Utc::now(),
            source_path: copy.working_path.to_string_lossy().to_string(),
            hdf5_path: String::new(),
            metadata: ArtifactMetadata {
                author: None,
                description: "Committed from working copy".to_string(),
                tags: vec!["committed".to_string()],
                parent_artifact_id: Some(copy.source_artifact_id.clone()),
                custom_fields: std::collections::HashMap::new(),
            },
            checksum: String::new(),
            size_bytes: 0,
        })
    }

    /// Discard working copy
    pub fn discard_working_copy(&self, copy: &mut WorkingCopy) -> anyhow::Result<()> {
        copy.status = WorkingCopyStatus::Discarded;

        // TODO: Implement actual cleanup
        // For now, just mark as discarded
        if copy.working_path.exists() {
            std::fs::remove_dir_all(&copy.working_path)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artifact_types() {
        let img = ArtifactType::Image {
            format: ImageFormat::PNG,
        };
        let json = serde_json::to_string(&img).unwrap();
        assert!(json.contains("Image"));
    }

    #[test]
    fn test_working_copy_creation() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let manager = CopyOnWorkManager::new(
            temp_dir.path().join("immutable"),
            temp_dir.path().join("working"),
        );

        let artifact = ImmutableArtifact {
            artifact_id: "test_001".to_string(),
            artifact_type: ArtifactType::SourceCode {
                language: "rust".to_string(),
            },
            version: "1.0.0".to_string(),
            created: Utc::now(),
            source_path: "src/main.rs".to_string(),
            hdf5_path: "/artifacts/test_001".to_string(),
            metadata: ArtifactMetadata {
                author: None,
                description: "Test artifact".to_string(),
                tags: vec![],
                parent_artifact_id: None,
                custom_fields: std::collections::HashMap::new(),
            },
            checksum: "abc123".to_string(),
            size_bytes: 1024,
        };

        let copy = manager.create_working_copy(&artifact, RunId::new(), Seq::zero())?;

        assert_eq!(copy.source_artifact_id, "test_001");
        assert!(matches!(copy.status, WorkingCopyStatus::Active));

        Ok(())
    }
}
