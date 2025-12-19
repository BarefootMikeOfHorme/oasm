/// Rule loader - loads rules from YAML templates and project configs

use super::{HierarchicalRule, RuleLevel, RuleSource};
use crate::{Condition, Rule, RuleCategory, Severity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Rule definition in YAML templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDefinition {
    pub id: String,
    pub program_type: String,
    pub category: String,
    pub level: String,
    pub overrides: Option<String>,
    pub enabled: Option<bool>,
    pub conditions: Vec<ConditionDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionDefinition {
    pub check_type: String,
    pub severity: String,
    pub message: String,
}

/// Rule loader
pub struct RuleLoader {
    template_paths: Vec<PathBuf>,
}

impl RuleLoader {
    pub fn new() -> Self {
        Self {
            template_paths: Vec::new(),
        }
    }

    /// Add a template path to search
    pub fn add_template_path(&mut self, path: PathBuf) {
        self.template_paths.push(path);
    }

    /// Load rules from YAML file
    pub fn load_from_yaml(&self, path: &PathBuf) -> Result<Vec<HierarchicalRule>, LoaderError> {
        // TODO: Implement actual YAML loading when serde_yaml is available
        // Check if file exists
        if !path.exists() {
            return Err(LoaderError::FileNotFound(path.clone()));
        }
        // For now, return empty vec until YAML parser is integrated
        Ok(Vec::new())
    }

    /// Load project-level rules from project config
    pub fn load_project_rules(&self, project_path: &PathBuf) -> Result<Vec<HierarchicalRule>, LoaderError> {
        let config_path = project_path.join("oasm.project.yaml");

        if !config_path.exists() {
            return Ok(Vec::new());
        }

        // TODO: Implement actual project config loading
        Ok(Vec::new())
    }

    /// Create a hierarchical rule from definition
    pub fn create_rule(&self, def: RuleDefinition, source: RuleSource) -> Result<HierarchicalRule, LoaderError> {
        let level = self.parse_level(&def.level)?;
        let category = self.parse_category(&def.category)?;
        let conditions = def.conditions
            .into_iter()
            .map(|c| self.parse_condition(c))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(HierarchicalRule {
            rule: Rule {
                id: def.id,
                program_type: def.program_type,
                category,
                conditions,
            },
            level,
            overrides: def.overrides,
            source,
            enabled: def.enabled.unwrap_or(true),
        })
    }

    fn parse_level(&self, level_str: &str) -> Result<RuleLevel, LoaderError> {
        match level_str.to_lowercase().as_str() {
            "core" => Ok(RuleLevel::Core),
            "domain" => Ok(RuleLevel::Domain),
            "project" => Ok(RuleLevel::Project),
            "session" => Ok(RuleLevel::Session),
            _ => Err(LoaderError::InvalidLevel(level_str.to_string())),
        }
    }

    fn parse_category(&self, category_str: &str) -> Result<RuleCategory, LoaderError> {
        match category_str.to_lowercase().as_str() {
            "validation" => Ok(RuleCategory::Validation),
            "behavior" => Ok(RuleCategory::Behavior),
            "constraint" => Ok(RuleCategory::Constraint),
            "output" => Ok(RuleCategory::Output),
            _ => Err(LoaderError::InvalidCategory(category_str.to_string())),
        }
    }

    fn parse_severity(&self, severity_str: &str) -> Result<Severity, LoaderError> {
        match severity_str.to_lowercase().as_str() {
            "error" => Ok(Severity::Error),
            "warning" => Ok(Severity::Warning),
            "info" => Ok(Severity::Info),
            _ => Err(LoaderError::InvalidSeverity(severity_str.to_string())),
        }
    }

    fn parse_condition(&self, cond_def: ConditionDefinition) -> Result<Condition, LoaderError> {
        Ok(Condition {
            check_type: cond_def.check_type,
            severity: self.parse_severity(&cond_def.severity)?,
            message: cond_def.message,
        })
    }
}

impl Default for RuleLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Loader errors
#[derive(Debug, Clone)]
pub enum LoaderError {
    FileNotFound(PathBuf),
    ParseError(String),
    InvalidLevel(String),
    InvalidCategory(String),
    InvalidSeverity(String),
    IoError(String),
}

impl std::fmt::Display for LoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoaderError::FileNotFound(path) => write!(f, "File not found: {:?}", path),
            LoaderError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            LoaderError::InvalidLevel(level) => write!(f, "Invalid rule level: {}", level),
            LoaderError::InvalidCategory(cat) => write!(f, "Invalid category: {}", cat),
            LoaderError::InvalidSeverity(sev) => write!(f, "Invalid severity: {}", sev),
            LoaderError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for LoaderError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_level() {
        let loader = RuleLoader::new();
        assert!(matches!(loader.parse_level("core"), Ok(RuleLevel::Core)));
        assert!(matches!(loader.parse_level("domain"), Ok(RuleLevel::Domain)));
        assert!(matches!(loader.parse_level("project"), Ok(RuleLevel::Project)));
        assert!(matches!(loader.parse_level("session"), Ok(RuleLevel::Session)));
        assert!(loader.parse_level("invalid").is_err());
    }

    #[test]
    fn test_parse_category() {
        let loader = RuleLoader::new();
        assert!(matches!(loader.parse_category("validation"), Ok(RuleCategory::Validation)));
        assert!(matches!(loader.parse_category("behavior"), Ok(RuleCategory::Behavior)));
        assert!(matches!(loader.parse_category("constraint"), Ok(RuleCategory::Constraint)));
        assert!(matches!(loader.parse_category("output"), Ok(RuleCategory::Output)));
    }

    #[test]
    fn test_create_rule() {
        let loader = RuleLoader::new();
        let def = RuleDefinition {
            id: "test_rule".to_string(),
            program_type: "cad".to_string(),
            category: "validation".to_string(),
            level: "project".to_string(),
            overrides: None,
            enabled: Some(true),
            conditions: vec![
                ConditionDefinition {
                    check_type: "test_check".to_string(),
                    severity: "error".to_string(),
                    message: "Test message".to_string(),
                },
            ],
        };

        let hrule = loader.create_rule(def, RuleSource::Builtin).unwrap();
        assert_eq!(hrule.rule.id, "test_rule");
        assert_eq!(hrule.level, RuleLevel::Project);
        assert_eq!(hrule.rule.category, RuleCategory::Validation);
    }
}
