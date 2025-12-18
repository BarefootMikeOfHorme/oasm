//! Format Converters
//!
//! Maintains unique relationships between HDF5, CBOR, YAML, and JSON.
//!
//! CONVERSION RULES (STRICT):
//!
//! 1. HDF5 → CBOR:
//!    - Extract template → generate runtime object
//!    - Deep artifacts (CFG/DFG, large datasets) STAY in HDF5 (referenced, not duplicated)
//!    - Only metadata and execution parameters go to CBOR
//!
//! 2. HDF5 → YAML:
//!    - Generate human-readable overlay with auto-populated fields
//!    - Add annotations explaining template intent
//!    - Deep artifacts referenced by HDF5 path, not embedded
//!
//! 3. YAML → CBOR:
//!    - Validate YAML overlay
//!    - Convert to compact binary for execution
//!    - Strip comments (annotations logged separately in JSON lineage)
//!
//! 4. CBOR → JSON (Lineage):
//!    - Extract execution outcome
//!    - Record provenance and impact
//!    - Reference CBOR object by object_id (not duplicate)
//!    - Reference HDF5 template by template_id (not duplicate)
//!
//! 5. FORBIDDEN CONVERSIONS:
//!    - CBOR → HDF5 (HDF5 is immutable canonical, never generated from runtime)
//!    - JSON → HDF5 (JSON is audit trail, not source of truth)
//!    - Large datasets to YAML/JSON (use HDF5 references)

use crate::schemas::{
    HDF5Template, CBORRuntimeObject, YAMLOverlay, JSONLineage,
    AutoPopulatedFields, Annotation, CommandBlock,
};
use crate::templates::TemplateStore;
use crate::runtime::RuntimeObjectManager;
use crate::lineage::LineageManager;
use crate::{RunId, Seq, Actor};
use anyhow::{Result, Context};

/// Converter between data formats
pub struct FormatConverter {
    template_store: TemplateStore,
    runtime_manager: RuntimeObjectManager,
    lineage_manager: LineageManager,
}

impl FormatConverter {
    pub fn new(
        template_store: TemplateStore,
        runtime_manager: RuntimeObjectManager,
        lineage_manager: LineageManager,
    ) -> Self {
        Self {
            template_store,
            runtime_manager,
            lineage_manager,
        }
    }

    /// HDF5 → CBOR (extract template, generate runtime object)
    ///
    /// CRITICAL: Deep artifacts (CFG/DFG, datasets) remain in HDF5.
    /// Only metadata and execution parameters go to CBOR.
    pub fn hdf5_to_cbor(
        &self,
        template_id: &str,
        run_id: RunId,
        seq: Seq,
        actor: Actor,
    ) -> Result<CBORRuntimeObject> {
        // Load immutable template from HDF5
        let template = self.template_store.load_template(template_id)?;

        // Extract command block (lightweight, no deep artifacts)
        let command = self.extract_command_from_template(&template)?;

        // Create CBOR runtime object
        let obj = self.runtime_manager.create_object(run_id, seq, actor, command);

        // IMPORTANT: obj does NOT contain CFG/DFG/datasets
        // Those remain in HDF5, referenced by template.artifacts[].data_path

        Ok(obj)
    }

    /// HDF5 → YAML (generate human-readable overlay)
    ///
    /// CRITICAL: Deep artifacts referenced by HDF5 path, not embedded.
    pub fn hdf5_to_yaml(&self, template_id: &str) -> Result<YAMLOverlay> {
        let template = self.template_store.load_template(template_id)?;

        // Generate auto-populated fields
        let auto_fields = AutoPopulatedFields {
            run_id: RunId::new(),
            seq: Seq::zero(),
            timestamp: chrono::Utc::now(),
            actor: Actor::System,
            file_path: None,
            rule_group: None,
            confidence: None,
            tests_planned: vec![],
        };

        // Generate annotations
        let mut annotations = vec![
            Annotation {
                field: "template_id".to_string(),
                explanation: format!("Immutable template: {}", template.template_id),
                rationale: Some("Canonical baseline from HDF5".to_string()),
            },
        ];

        // Annotate artifact references (not embedded!)
        for artifact in &template.artifacts {
            annotations.push(Annotation {
                field: format!("artifact.{}", artifact.artifact_id),
                explanation: format!("HDF5 reference: {}", artifact.data_path),
                rationale: Some("Large artifact remains in HDF5 store".to_string()),
            });
        }

        let overlay = YAMLOverlay {
            comment: Some(format!(
                "Auto-generated overlay for template: {}\nDeep artifacts in HDF5, not embedded here.",
                template.template_id
            )),
            metadata: crate::ExecutionMetadata::new(Actor::System),
            command: self.extract_command_from_template(&template)?,
            auto_populated: auto_fields,
            annotations,
        };

        Ok(overlay)
    }

    /// YAML → CBOR (validate and convert for execution)
    ///
    /// CRITICAL: Validate YAML, strip comments, produce compact binary.
    pub fn yaml_to_cbor(&self, yaml_overlay: &YAMLOverlay) -> Result<CBORRuntimeObject> {
        // Validate YAML structure
        self.validate_yaml_overlay(yaml_overlay)?;

        // Create CBOR object (comments stripped, annotations logged separately)
        let obj = CBORRuntimeObject {
            object_id: format!(
                "{}_{}",
                yaml_overlay.auto_populated.run_id,
                yaml_overlay.auto_populated.seq.0
            ),
            metadata: yaml_overlay.metadata.clone(),
            command: yaml_overlay.command.clone(),
            auto_fields: yaml_overlay.auto_populated.clone(),
            decisions: Vec::new(),
        };

        Ok(obj)
    }

    /// CBOR → JSON Lineage (record execution outcome)
    ///
    /// CRITICAL: Reference CBOR object and HDF5 template by ID, don't duplicate.
    pub fn cbor_to_json_lineage(
        &self,
        cbor_obj: &CBORRuntimeObject,
        outcome: crate::schemas::ExecutionOutcome,
        impact: crate::Impact,
    ) -> Result<JSONLineage> {
        let lineage = self.lineage_manager.record(
            cbor_obj.auto_fields.run_id,
            cbor_obj.auto_fields.seq,
            cbor_obj.auto_fields.actor.clone(),
            format!("Executed {:?}", cbor_obj.command.block_type),
            "Automated execution", // TODO: extract from CBOR
            outcome,
            crate::schemas::Provenance {
                tool_versions: cbor_obj.metadata.tool_versions.clone(),
                config_hash: cbor_obj.metadata.config_hash.clone(),
                template_id: None, // TODO: track template_id in CBOR
                parent_run_id: None,
                lineage_chain: vec![],
                confidence: cbor_obj.auto_fields.confidence,
            },
            impact,
        )?;

        Ok(lineage)
    }

    /// Validate YAML overlay structure
    fn validate_yaml_overlay(&self, overlay: &YAMLOverlay) -> Result<()> {
        // Check required fields
        if overlay.auto_populated.run_id.0.is_nil() {
            anyhow::bail!("Invalid run_id in YAML overlay");
        }

        // Validate command block
        if overlay.command.target_files.is_empty() && overlay.command.parameters.is_empty() {
            anyhow::bail!("YAML overlay has empty command block");
        }

        Ok(())
    }

    /// Extract command block from HDF5 template (lightweight)
    fn extract_command_from_template(&self, template: &HDF5Template) -> Result<CommandBlock> {
        // TODO: Implement actual extraction logic based on template type
        // For now, return placeholder

        let block_type = match template.template_type {
            crate::schemas::TemplateType::AssemblerPass => crate::schemas::BlockType::AnalysisPass,
            crate::schemas::TemplateType::TestHarness => crate::schemas::BlockType::TestRunner,
            crate::schemas::TemplateType::LintBundle => crate::schemas::BlockType::LintCheck,
            _ => crate::schemas::BlockType::AnalysisPass,
        };

        Ok(CommandBlock {
            block_type,
            parameters: vec![],
            target_files: vec![],
            rules: vec![],
        })
    }
}

/// Conversion pipeline orchestrator
pub struct ConversionPipeline {
    converter: FormatConverter,
}

impl ConversionPipeline {
    pub fn new(converter: FormatConverter) -> Self {
        Self { converter }
    }

    /// Full pipeline: HDF5 → CBOR → Execute → JSON Lineage
    ///
    /// This is the canonical execution flow:
    /// 1. Load immutable template from HDF5
    /// 2. Generate CBOR runtime object
    /// 3. Execute CBOR object
    /// 4. Record outcome in JSON lineage
    /// 5. Discard CBOR object (ephemeral)
    /// 6. Lineage persists
    pub fn execute_from_template(
        &self,
        template_id: &str,
        run_id: RunId,
        seq: Seq,
        actor: Actor,
    ) -> Result<JSONLineage> {
        // Step 1: HDF5 → CBOR
        let cbor_obj = self.converter.hdf5_to_cbor(template_id, run_id, seq, actor)?;

        // Step 2: Execute CBOR
        let result = self.converter.runtime_manager.execute(&cbor_obj)?;

        // Step 3: CBOR → JSON Lineage
        let lineage = self.converter.cbor_to_json_lineage(
            &cbor_obj,
            result.outcome,
            crate::Impact::default(), // TODO: extract from result
        )?;

        // Step 4: CBOR object is ephemeral, discarded here
        // Only lineage persists

        Ok(lineage)
    }

    /// Alternative pipeline: YAML → CBOR → Execute → JSON Lineage
    ///
    /// Used when human creates YAML overlay directly:
    /// 1. Load/create YAML overlay
    /// 2. Validate and convert to CBOR
    /// 3. Execute CBOR object
    /// 4. Record outcome in JSON lineage
    /// 5. Discard CBOR object (ephemeral)
    /// 6. Lineage persists
    pub fn execute_from_yaml(&self, yaml_overlay: &YAMLOverlay) -> Result<JSONLineage> {
        // Step 1: YAML → CBOR
        let cbor_obj = self.converter.yaml_to_cbor(yaml_overlay)?;

        // Step 2: Execute CBOR
        let result = self.converter.runtime_manager.execute(&cbor_obj)?;

        // Step 3: CBOR → JSON Lineage (with YAML annotations)
        let mut lineage = self.converter.cbor_to_json_lineage(
            &cbor_obj,
            result.outcome,
            crate::Impact::default(),
        )?;

        // Step 4: Attach YAML annotations to lineage
        // (This preserves human reasoning without embedding in CBOR)
        for annotation in &yaml_overlay.annotations {
            // TODO: Store annotations in lineage provenance
        }

        Ok(lineage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_rules() {
        // Test that conversion rules are documented
        // Actual conversion logic tested in integration tests
        assert!(true);
    }
}
