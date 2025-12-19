/// Hierarchical Rule Engine for OASM
/// Implements Core → Domain → Project → Session hierarchy (most specific wins)

pub mod hierarchy;
pub mod loader;
pub mod resolver;

use crate::{Rule, RuleCategory, Condition, Severity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Rule hierarchy levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RuleLevel {
    Core = 0,        // System-level rules (type safety, memory safety)
    Domain = 1,      // Program-type rules (CAD topology, engine scene graph)
    Project = 2,     // Project-specific rules
    Session = 3,     // Runtime/user rules (highest priority)
}

impl RuleLevel {
    pub fn priority(&self) -> u8 {
        *self as u8
    }
}

/// Hierarchical rule with level and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchicalRule {
    pub rule: Rule,
    pub level: RuleLevel,
    pub overrides: Option<String>,  // Rule ID this overrides
    pub source: RuleSource,
    pub enabled: bool,
}

/// Rule source tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuleSource {
    Builtin,
    Template { path: String },
    ProjectConfig { path: String },
    UserDefined { session_id: String },
}

/// Hierarchical rule engine
pub struct HierarchicalRuleEngine {
    rules: HashMap<String, HierarchicalRule>,
    level_index: HashMap<RuleLevel, Vec<String>>,  // Level -> Rule IDs
    program_index: HashMap<String, Vec<String>>,   // Program type -> Rule IDs
}

impl HierarchicalRuleEngine {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            level_index: HashMap::new(),
            program_index: HashMap::new(),
        }
    }

    /// Register a hierarchical rule
    pub fn register_rule(&mut self, hrule: HierarchicalRule) {
        let rule_id = hrule.rule.id.clone();
        let level = hrule.level;
        let program_type = hrule.rule.program_type.clone();

        // Add to main registry
        self.rules.insert(rule_id.clone(), hrule);

        // Add to level index
        self.level_index
            .entry(level)
            .or_insert_with(Vec::new)
            .push(rule_id.clone());

        // Add to program index
        self.program_index
            .entry(program_type)
            .or_insert_with(Vec::new)
            .push(rule_id);
    }

    /// Get rules for a program type with hierarchy resolution
    pub fn get_resolved_rules(&self, program_type: &str) -> Vec<&HierarchicalRule> {
        let mut rules = Vec::new();
        let mut overridden = HashMap::new();

        // Get all rules for this program type
        if let Some(rule_ids) = self.program_index.get(program_type) {
            let mut hrules: Vec<_> = rule_ids
                .iter()
                .filter_map(|id| self.rules.get(id))
                .filter(|hr| hr.enabled)
                .collect();

            // Sort by level (Session > Project > Domain > Core)
            hrules.sort_by(|a, b| b.level.cmp(&a.level));

            // Process overrides
            for hrule in &hrules {
                if let Some(overrides_id) = &hrule.overrides {
                    overridden.insert(overrides_id.clone(), true);
                }
            }

            // Collect non-overridden rules
            for hrule in hrules {
                if !overridden.contains_key(&hrule.rule.id) {
                    rules.push(hrule);
                }
            }
        }

        rules
    }

    /// Get rules by level
    pub fn get_rules_by_level(&self, level: RuleLevel) -> Vec<&HierarchicalRule> {
        if let Some(rule_ids) = self.level_index.get(&level) {
            rule_ids
                .iter()
                .filter_map(|id| self.rules.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Disable a rule
    pub fn disable_rule(&mut self, rule_id: &str) -> Result<(), RuleEngineError> {
        if let Some(hrule) = self.rules.get_mut(rule_id) {
            hrule.enabled = false;
            Ok(())
        } else {
            Err(RuleEngineError::RuleNotFound(rule_id.to_string()))
        }
    }

    /// Enable a rule
    pub fn enable_rule(&mut self, rule_id: &str) -> Result<(), RuleEngineError> {
        if let Some(hrule) = self.rules.get_mut(rule_id) {
            hrule.enabled = true;
            Ok(())
        } else {
            Err(RuleEngineError::RuleNotFound(rule_id.to_string()))
        }
    }

    /// Validate data against rules
    pub fn validate(
        &self,
        program_type: &str,
        data: &HashMap<String, String>,
    ) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut info = Vec::new();

        let rules = self.get_resolved_rules(program_type);

        for hrule in rules {
            for condition in &hrule.rule.conditions {
                // TODO: Implement actual condition checking
                // For now, placeholder logic
                match condition.severity {
                    Severity::Error => {
                        if data.is_empty() {
                            errors.push(ValidationMessage {
                                rule_id: hrule.rule.id.clone(),
                                level: hrule.level,
                                severity: Severity::Error,
                                message: condition.message.clone(),
                                check_type: condition.check_type.clone(),
                            });
                        }
                    }
                    Severity::Warning => {}
                    Severity::Info => {}
                }
            }
        }

        ValidationResult {
            passed: errors.is_empty(),
            errors,
            warnings,
            info,
        }
    }
}

impl Default for HierarchicalRuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub passed: bool,
    pub errors: Vec<ValidationMessage>,
    pub warnings: Vec<ValidationMessage>,
    pub info: Vec<ValidationMessage>,
}

/// Validation message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationMessage {
    pub rule_id: String,
    pub level: RuleLevel,
    pub severity: Severity,
    pub message: String,
    pub check_type: String,
}

/// Rule engine errors
#[derive(Debug, Clone)]
pub enum RuleEngineError {
    RuleNotFound(String),
    InvalidOverride { rule_id: String, overrides: String },
    ConflictingRules { rule1: String, rule2: String },
}

impl std::fmt::Display for RuleEngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuleEngineError::RuleNotFound(id) => write!(f, "Rule not found: {}", id),
            RuleEngineError::InvalidOverride { rule_id, overrides } => {
                write!(f, "Rule {} cannot override {}", rule_id, overrides)
            }
            RuleEngineError::ConflictingRules { rule1, rule2 } => {
                write!(f, "Conflicting rules: {} and {}", rule1, rule2)
            }
        }
    }
}

impl std::error::Error for RuleEngineError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_retrieve() {
        let mut engine = HierarchicalRuleEngine::new();

        let hrule = HierarchicalRule {
            rule: Rule {
                id: "core_type_safety".to_string(),
                program_type: "all".to_string(),
                category: RuleCategory::Validation,
                conditions: vec![],
            },
            level: RuleLevel::Core,
            overrides: None,
            source: RuleSource::Builtin,
            enabled: true,
        };

        engine.register_rule(hrule);

        let core_rules = engine.get_rules_by_level(RuleLevel::Core);
        assert_eq!(core_rules.len(), 1);
    }

    #[test]
    fn test_override_resolution() {
        let mut engine = HierarchicalRuleEngine::new();

        // Core rule
        let core_rule = HierarchicalRule {
            rule: Rule {
                id: "core_max_depth".to_string(),
                program_type: "cad".to_string(),
                category: RuleCategory::Constraint,
                conditions: vec![],
            },
            level: RuleLevel::Core,
            overrides: None,
            source: RuleSource::Builtin,
            enabled: true,
        };

        // Session rule that overrides core
        let session_rule = HierarchicalRule {
            rule: Rule {
                id: "session_max_depth".to_string(),
                program_type: "cad".to_string(),
                category: RuleCategory::Constraint,
                conditions: vec![],
            },
            level: RuleLevel::Session,
            overrides: Some("core_max_depth".to_string()),
            source: RuleSource::UserDefined {
                session_id: "test_session".to_string(),
            },
            enabled: true,
        };

        engine.register_rule(core_rule);
        engine.register_rule(session_rule);

        let resolved = engine.get_resolved_rules("cad");
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].rule.id, "session_max_depth");
    }
}
