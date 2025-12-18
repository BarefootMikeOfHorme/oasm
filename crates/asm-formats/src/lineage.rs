//! JSON Lineage Tracking
//!
//! Audit trails and provenance logs in Git-friendly JSON format.
//! Records execution outcomes, decisions, and lineage chains.
//!
//! UNIQUE RELATIONSHIP WITH OTHER FORMATS:
//! - HDF5: References immutable templates, never duplicates deep artifacts
//! - CBOR: Records execution of CBOR objects (object_id reference)
//! - YAML: Captures annotations and human decisions from overlays
//! - JSON: Standalone format optimized for Git diffs and audit trails

use crate::schemas::{JSONLineage, ExecutionOutcome, Provenance, TestRecord, DiffSnapshot};
use crate::{RunId, Seq, Actor, Impact};
use anyhow::Result;
use std::path::Path;
use chrono::Utc;

/// Lineage manager for tracking execution history
pub struct LineageManager {
    lineage_dir: std::path::PathBuf,
}

impl LineageManager {
    pub fn new(lineage_dir: impl AsRef<Path>) -> Self {
        Self {
            lineage_dir: lineage_dir.as_ref().to_path_buf(),
        }
    }

    /// Record a new lineage entry
    pub fn record(
        &self,
        run_id: RunId,
        seq: Seq,
        actor: Actor,
        summary: impl Into<String>,
        intent: impl Into<String>,
        outcome: ExecutionOutcome,
        provenance: Provenance,
        impact: Impact,
    ) -> Result<JSONLineage> {
        let lineage_id = format!("{}_{}", run_id, seq.0);

        let lineage = JSONLineage {
            lineage_id: lineage_id.clone(),
            run_id,
            seq,
            timestamp: Utc::now(),
            actor,
            summary: summary.into(),
            intent: intent.into(),
            command_executed: String::new(), // Populated by caller
            outcome,
            provenance,
            impact,
            tests: Vec::new(),
            diff_id: None,
            git_sha: None,
        };

        self.save(&lineage)?;

        Ok(lineage)
    }

    /// Save lineage entry to disk (JSON format, Git-friendly)
    pub fn save(&self, lineage: &JSONLineage) -> Result<()> {
        std::fs::create_dir_all(&self.lineage_dir)?;

        // Organize by run_id for easy browsing
        let run_dir = self.lineage_dir.join(lineage.run_id.to_string());
        std::fs::create_dir_all(&run_dir)?;

        let path = run_dir.join(format!("seq_{:04}.json", lineage.seq.0));

        // Pretty JSON for Git-friendly diffs
        let json = serde_json::to_string_pretty(lineage)?;
        std::fs::write(path, json)?;

        Ok(())
    }

    /// Load lineage entry
    pub fn load(&self, run_id: RunId, seq: Seq) -> Result<JSONLineage> {
        let path = self.lineage_dir
            .join(run_id.to_string())
            .join(format!("seq_{:04}.json", seq.0));

        let json = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }

    /// Get all lineage entries for a run
    pub fn get_run_lineage(&self, run_id: RunId) -> Result<Vec<JSONLineage>> {
        let run_dir = self.lineage_dir.join(run_id.to_string());

        let mut entries = Vec::new();

        for entry in std::fs::read_dir(run_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let json = std::fs::read_to_string(&path)?;
                let lineage: JSONLineage = serde_json::from_str(&json)?;
                entries.push(lineage);
            }
        }

        // Sort by sequence
        entries.sort_by_key(|e| e.seq);

        Ok(entries)
    }

    /// Build lineage chain (parent → child relationships)
    pub fn build_lineage_chain(&self, run_id: RunId) -> Result<Vec<String>> {
        let entries = self.get_run_lineage(run_id)?;

        let mut chain = Vec::new();
        for entry in entries {
            chain.push(format!(
                "{:04}: {} → {}",
                entry.seq.0,
                entry.intent,
                match entry.outcome {
                    ExecutionOutcome::Success => "✓",
                    ExecutionOutcome::Failed { .. } => "✗",
                    ExecutionOutcome::PartialSuccess { .. } => "⚠",
                    ExecutionOutcome::Cancelled => "⊗",
                }
            ));
        }

        Ok(chain)
    }

    /// Add test record to lineage entry
    pub fn add_test_record(
        &self,
        run_id: RunId,
        seq: Seq,
        test_record: TestRecord,
    ) -> Result<()> {
        let mut lineage = self.load(run_id, seq)?;
        lineage.tests.push(test_record);
        self.save(&lineage)?;
        Ok(())
    }

    /// Link diff to lineage entry
    pub fn link_diff(&self, run_id: RunId, seq: Seq, diff_id: String) -> Result<()> {
        let mut lineage = self.load(run_id, seq)?;
        lineage.diff_id = Some(diff_id);
        self.save(&lineage)?;
        Ok(())
    }

    /// Link Git SHA to lineage entry
    pub fn link_git_sha(&self, run_id: RunId, seq: Seq, git_sha: String) -> Result<()> {
        let mut lineage = self.load(run_id, seq)?;
        lineage.git_sha = Some(git_sha);
        self.save(&lineage)?;
        Ok(())
    }
}

/// Diff snapshot manager (unified diff format)
pub struct DiffManager {
    diffs_dir: std::path::PathBuf,
}

impl DiffManager {
    pub fn new(diffs_dir: impl AsRef<Path>) -> Self {
        Self {
            diffs_dir: diffs_dir.as_ref().to_path_buf(),
        }
    }

    /// Save diff snapshot
    pub fn save_diff(&self, diff: &DiffSnapshot) -> Result<()> {
        std::fs::create_dir_all(&self.diffs_dir)?;

        // Organize by run_id
        let run_dir = self.diffs_dir.join(diff.header.run_id.to_string());
        std::fs::create_dir_all(&run_dir)?;

        let path = run_dir.join(format!("{}.diff.yaml", diff.header.diff_id));

        // YAML format for diffs (header + hunks)
        let yaml = serde_yaml::to_string(diff)?;
        std::fs::write(path, yaml)?;

        Ok(())
    }

    /// Load diff snapshot
    pub fn load_diff(&self, run_id: RunId, diff_id: &str) -> Result<DiffSnapshot> {
        let path = self.diffs_dir
            .join(run_id.to_string())
            .join(format!("{}.diff.yaml", diff_id));

        let yaml = std::fs::read_to_string(path)?;
        Ok(serde_yaml::from_str(&yaml)?)
    }

    /// Apply diff (preview mode)
    pub fn preview_diff(&self, diff: &DiffSnapshot) -> String {
        let mut output = String::new();

        output.push_str(&format!("=== Diff {} ===\n", diff.header.diff_id));
        output.push_str(&format!("Summary: {}\n", diff.header.summary));
        output.push_str(&format!("Confidence: {}\n", diff.header.confidence.0));
        output.push_str(&format!("Impact: +{} -{} files\n",
            diff.header.impact.lines_added,
            diff.header.impact.lines_removed
        ));
        output.push('\n');

        for hunk in &diff.hunks {
            output.push_str(&format!("--- {}\n", hunk.file_path));
            output.push_str(&format!("+++ {}\n", hunk.file_path));
            output.push_str(&format!("@@ -{},{} +{},{} @@\n",
                hunk.old_start, hunk.old_count,
                hunk.new_start, hunk.new_count
            ));

            for line in &hunk.lines {
                let prefix = match line.line_type {
                    crate::schemas::DiffLineType::Context => " ",
                    crate::schemas::DiffLineType::Removal => "-",
                    crate::schemas::DiffLineType::Addition => "+",
                };
                output.push_str(&format!("{}{}\n", prefix, line.content));
            }

            output.push('\n');
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Confidence;

    #[test]
    fn test_lineage_recording() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let manager = LineageManager::new(temp_dir.path());

        let run_id = RunId::new();
        let seq = Seq::zero();

        let lineage = manager.record(
            run_id,
            seq,
            Actor::System,
            "Test operation",
            "Testing lineage",
            ExecutionOutcome::Success,
            Provenance {
                tool_versions: crate::ToolVersions::current(),
                config_hash: "abc123".to_string(),
                template_id: Some("template_001".to_string()),
                parent_run_id: None,
                lineage_chain: vec![],
                confidence: Some(Confidence::high()),
            },
            Impact::default(),
        )?;

        assert_eq!(lineage.run_id, run_id);
        assert_eq!(lineage.seq, seq);

        let loaded = manager.load(run_id, seq)?;
        assert_eq!(loaded.summary, "Test operation");

        Ok(())
    }

    #[test]
    fn test_lineage_chain() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let manager = LineageManager::new(temp_dir.path());

        let run_id = RunId::new();

        // Record multiple steps
        for i in 0..3 {
            manager.record(
                run_id,
                Seq(i),
                Actor::System,
                format!("Step {}", i),
                format!("Intent {}", i),
                ExecutionOutcome::Success,
                Provenance {
                    tool_versions: crate::ToolVersions::current(),
                    config_hash: "abc123".to_string(),
                    template_id: None,
                    parent_run_id: None,
                    lineage_chain: vec![],
                    confidence: None,
                },
                Impact::default(),
            )?;
        }

        let chain = manager.build_lineage_chain(run_id)?;
        assert_eq!(chain.len(), 3);

        Ok(())
    }
}
