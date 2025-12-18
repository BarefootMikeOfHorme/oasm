//! HDF5 Template Management
//!
//! Immutable canonical templates stored in HDF5 format.
//! Provides baseline snapshots and deep artifacts (CFG/DFG, test fixtures, datasets).

use crate::schemas::{HDF5Template, TemplateType, Artifact, BaselineSnapshot};
use anyhow::{Result, Context};
use std::path::Path;

/// Template store backed by HDF5
pub struct TemplateStore {
    base_path: std::path::PathBuf,
}

impl TemplateStore {
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    /// Load an immutable template by ID
    pub fn load_template(&self, template_id: &str) -> Result<HDF5Template> {
        let template_path = self.base_path.join(format!("{}.h5", template_id));

        // TODO: Implement actual HDF5 reading
        // For now, return a placeholder
        anyhow::bail!("HDF5 reading not yet implemented for: {}", template_path.display())
    }

    /// Store a new immutable template
    pub fn store_template(&self, template: &HDF5Template) -> Result<()> {
        let template_path = self.base_path.join(format!("{}.h5", template.template_id));

        // TODO: Implement actual HDF5 writing
        // For now, serialize to JSON as placeholder
        let json = serde_json::to_string_pretty(template)?;
        std::fs::write(
            template_path.with_extension("json"),
            json
        )?;

        Ok(())
    }

    /// List all available templates
    pub fn list_templates(&self) -> Result<Vec<String>> {
        let mut templates = Vec::new();

        for entry in std::fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("h5") ||
               path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    templates.push(stem.to_string());
                }
            }
        }

        Ok(templates)
    }

    /// Get template by type
    pub fn find_by_type(&self, template_type: TemplateType) -> Result<Vec<String>> {
        // TODO: Implement actual filtering
        // For now, list all templates
        self.list_templates()
    }
}

/// Template builder for creating new immutable templates
pub struct TemplateBuilder {
    template: HDF5Template,
}

impl TemplateBuilder {
    pub fn new(template_id: impl Into<String>, template_type: TemplateType) -> Self {
        Self {
            template: HDF5Template {
                template_id: template_id.into(),
                version: "1.0.0".to_string(),
                created: chrono::Utc::now(),
                description: String::new(),
                template_type,
                artifacts: Vec::new(),
                baseline: BaselineSnapshot {
                    snapshot_id: format!("baseline_{}", uuid::Uuid::new_v4()),
                    timestamp: chrono::Utc::now(),
                    files: Vec::new(),
                    metrics: crate::schemas::BaselineMetrics {
                        total_files: 0,
                        total_loc: 0,
                        total_functions: 0,
                        complexity_score: 0.0,
                    },
                },
            },
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.template.description = desc.into();
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.template.version = version.into();
        self
    }

    pub fn add_artifact(mut self, artifact: Artifact) -> Self {
        self.template.artifacts.push(artifact);
        self
    }

    pub fn baseline(mut self, baseline: BaselineSnapshot) -> Self {
        self.template.baseline = baseline;
        self
    }

    pub fn build(self) -> HDF5Template {
        self.template
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_builder() {
        let template = TemplateBuilder::new("test_template", TemplateType::AssemblerPass)
            .description("Test assembler pass")
            .version("1.0.0")
            .build();

        assert_eq!(template.template_id, "test_template");
        assert_eq!(template.description, "Test assembler pass");
    }

    #[test]
    fn test_template_store() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let store = TemplateStore::new(temp_dir.path());

        let template = TemplateBuilder::new("test_001", TemplateType::LintBundle)
            .description("Test lint bundle")
            .build();

        store.store_template(&template)?;

        let templates = store.list_templates()?;
        assert!(templates.contains(&"test_001".to_string()));

        Ok(())
    }
}
