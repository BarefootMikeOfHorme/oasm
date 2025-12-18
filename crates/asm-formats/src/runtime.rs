//! CBOR Runtime Objects
//!
//! Binary execution units generated from HDF5 templates or YAML overlays.
//! Compact, deterministic, immutable once created for a run.

use crate::schemas::{CBORRuntimeObject, CommandBlock, BlockType, AutoPopulatedFields};
use crate::{RunId, Seq, Actor, ExecutionMetadata};
use anyhow::Result;
use std::path::Path;

/// Runtime object manager
pub struct RuntimeObjectManager {
    cache_dir: std::path::PathBuf,
}

impl RuntimeObjectManager {
    pub fn new(cache_dir: impl AsRef<Path>) -> Self {
        Self {
            cache_dir: cache_dir.as_ref().to_path_buf(),
        }
    }

    /// Create a new runtime object
    pub fn create_object(
        &self,
        run_id: RunId,
        seq: Seq,
        actor: Actor,
        command: CommandBlock,
    ) -> CBORRuntimeObject {
        let object_id = format!("{}_{}", run_id, seq.0);

        CBORRuntimeObject {
            object_id,
            metadata: ExecutionMetadata::new(actor.clone()),
            command,
            auto_fields: AutoPopulatedFields {
                run_id,
                seq,
                timestamp: chrono::Utc::now(),
                actor,
                file_path: None,
                rule_group: None,
                confidence: None,
                tests_planned: Vec::new(),
            },
            decisions: Vec::new(),
        }
    }

    /// Serialize to CBOR bytes
    pub fn to_cbor(&self, obj: &CBORRuntimeObject) -> Result<Vec<u8>> {
        Ok(serde_cbor::to_vec(obj)?)
    }

    /// Deserialize from CBOR bytes
    pub fn from_cbor(&self, bytes: &[u8]) -> Result<CBORRuntimeObject> {
        Ok(serde_cbor::from_slice(bytes)?)
    }

    /// Save runtime object to cache
    pub fn save_object(&self, obj: &CBORRuntimeObject) -> Result<()> {
        std::fs::create_dir_all(&self.cache_dir)?;

        let path = self.cache_dir.join(format!("{}.cbor", obj.object_id));
        let bytes = self.to_cbor(obj)?;
        std::fs::write(path, bytes)?;

        Ok(())
    }

    /// Load runtime object from cache
    pub fn load_object(&self, object_id: &str) -> Result<CBORRuntimeObject> {
        let path = self.cache_dir.join(format!("{}.cbor", object_id));
        let bytes = std::fs::read(path)?;
        self.from_cbor(&bytes)
    }

    /// Execute a runtime object
    pub fn execute(&self, obj: &CBORRuntimeObject) -> Result<ExecutionResult> {
        // TODO: Implement actual execution logic
        // This would dispatch to appropriate handlers based on block_type

        Ok(ExecutionResult {
            object_id: obj.object_id.clone(),
            run_id: obj.auto_fields.run_id,
            seq: obj.auto_fields.seq,
            outcome: crate::schemas::ExecutionOutcome::Success,
            duration_ms: 0,
            logs: vec![],
        })
    }
}

/// Result of executing a runtime object
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub object_id: String,
    pub run_id: RunId,
    pub seq: Seq,
    pub outcome: crate::schemas::ExecutionOutcome,
    pub duration_ms: u64,
    pub logs: Vec<String>,
}

/// Command block builder
pub struct CommandBlockBuilder {
    block_type: BlockType,
    parameters: Vec<crate::schemas::Parameter>,
    target_files: Vec<String>,
    rules: Vec<String>,
}

impl CommandBlockBuilder {
    pub fn new(block_type: BlockType) -> Self {
        Self {
            block_type,
            parameters: Vec::new(),
            target_files: Vec::new(),
            rules: Vec::new(),
        }
    }

    pub fn parameter(mut self, key: impl Into<String>, value: crate::schemas::ParameterValue) -> Self {
        self.parameters.push(crate::schemas::Parameter {
            key: key.into(),
            value,
        });
        self
    }

    pub fn target_file(mut self, file: impl Into<String>) -> Self {
        self.target_files.push(file.into());
        self
    }

    pub fn rule(mut self, rule: impl Into<String>) -> Self {
        self.rules.push(rule.into());
        self
    }

    pub fn build(self) -> CommandBlock {
        CommandBlock {
            block_type: self.block_type,
            parameters: self.parameters,
            target_files: self.target_files,
            rules: self.rules,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_object_creation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let manager = RuntimeObjectManager::new(temp_dir.path());

        let command = CommandBlockBuilder::new(BlockType::LintCheck)
            .target_file("src/main.rs")
            .rule("no_unsafe")
            .build();

        let obj = manager.create_object(
            RunId::new(),
            Seq::zero(),
            Actor::System,
            command,
        );

        assert_eq!(obj.auto_fields.seq, Seq::zero());
    }

    #[test]
    fn test_cbor_serialization() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let manager = RuntimeObjectManager::new(temp_dir.path());

        let command = CommandBlockBuilder::new(BlockType::TestRunner).build();
        let obj = manager.create_object(
            RunId::new(),
            Seq::zero(),
            Actor::System,
            command,
        );

        let bytes = manager.to_cbor(&obj)?;
        let deserialized = manager.from_cbor(&bytes)?;

        assert_eq!(obj.object_id, deserialized.object_id);

        Ok(())
    }
}
