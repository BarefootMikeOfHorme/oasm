use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use std::fs;

/// A template loaded from a YAML file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub version: String,
    pub template_type: String,
    pub description: Option<String>,
    pub structure: Option<String>,
    pub placeholders: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_yaml::Value>,
}

/// Manages loading and caching of templates
pub struct TemplateManager {
    template_dir: PathBuf,
    cache: HashMap<String, Template>,
}

impl TemplateManager {
    pub fn new(template_dir: PathBuf) -> Self {
        Self {
            template_dir,
            cache: HashMap::new(),
        }
    }

    /// Loads a template from a relative path within the template directory
    pub fn load_template(&mut self, relative_path: &str) -> Result<Template> {
        if let Some(template) = self.cache.get(relative_path) {
            return Ok(template.clone());
        }

        let full_path = self.template_dir.join(relative_path);
        let content = fs::read_to_string(&full_path)
            .with_context(|| format!("Failed to read template file: {:?}", full_path))?;

        let template: Template = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse YAML template: {:?}", full_path))?;

        self.cache.insert(relative_path.to_string(), template.clone());
        Ok(template)
    }

    /// Lists templates in a specific category
    pub fn list_templates(&self, category: &str) -> Result<Vec<PathBuf>> {
        let category_path = self.template_dir.join(category);
        if !category_path.exists() {
            return Ok(Vec::new());
        }

        let mut templates = Vec::new();
        for entry in fs::read_dir(category_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "yaml" || ext == "yml") {
                templates.push(path.strip_prefix(&self.template_dir)?.to_path_buf());
            }
        }
        Ok(templates)
    }
}

/// Handles instantiation of templates by replacing placeholders
pub struct TemplateInstantiator;

impl TemplateInstantiator {
    /// Replaces placeholders in a string with provided values
    pub fn instantiate_string(
        content: &str,
        placeholders: &HashMap<String, String>,
    ) -> String {
        let mut instantiated = content.to_string();
        for (key, value) in placeholders {
            let pattern = format!("{{{}}}", key);
            instantiated = instantiated.replace(&pattern, value);
        }
        instantiated
    }

    /// Instantiates a template into a directory
    pub fn instantiate_to_file(
        template: &Template,
        placeholders: &HashMap<String, String>,
        output_path: &Path,
    ) -> Result<()> {
        if let Some(structure) = &template.structure {
            let content = Self::instantiate_string(structure, placeholders);
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(output_path, content)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Template '{}' has no structure to instantiate", template.name))
        }
    }
}
