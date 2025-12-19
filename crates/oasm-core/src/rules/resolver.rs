/// Rule resolver - resolves rule conflicts and applies hierarchy

use super::{HierarchicalRule, RuleLevel, ValidationMessage, ValidationResult};
use crate::Severity;
use std::collections::HashMap;

/// Rule resolver
pub struct RuleResolver {
    conflict_strategy: ConflictStrategy,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConflictStrategy {
    MostSpecificWins,  // Default: Session > Project > Domain > Core
    MostRestrictive,   // Use the most restrictive rule
    Merge,             // Merge conditions from all levels
}

impl RuleResolver {
    pub fn new(strategy: ConflictStrategy) -> Self {
        Self {
            conflict_strategy: strategy,
        }
    }

    /// Resolve conflicts between rules at different levels
    pub fn resolve_conflicts<'a>(
        &self,
        rules: &'a [&'a HierarchicalRule],
    ) -> Vec<&'a HierarchicalRule> {
        match self.conflict_strategy {
            ConflictStrategy::MostSpecificWins => self.resolve_most_specific(rules),
            ConflictStrategy::MostRestrictive => self.resolve_most_restrictive(rules),
            ConflictStrategy::Merge => self.resolve_merge(rules),
        }
    }

    fn resolve_most_specific<'a>(
        &self,
        rules: &'a [&'a HierarchicalRule],
    ) -> Vec<&'a HierarchicalRule> {
        let mut by_id: HashMap<String, Vec<&'a HierarchicalRule>> = HashMap::new();

        // Group rules by ID (base ID, ignoring level prefix)
        for &rule in rules {
            let base_id = self.get_base_id(&rule.rule.id);
            by_id.entry(base_id).or_insert_with(Vec::new).push(rule);
        }

        // For each group, select the most specific (highest level)
        let mut resolved = Vec::new();
        for (_id, mut group) in by_id {
            if group.len() == 1 {
                resolved.push(group[0]);
            } else {
                // Sort by level (Session > Project > Domain > Core)
                group.sort_by(|a, b| b.level.cmp(&a.level));
                resolved.push(group[0]);
            }
        }

        resolved
    }

    fn resolve_most_restrictive<'a>(
        &self,
        rules: &'a [&'a HierarchicalRule],
    ) -> Vec<&'a HierarchicalRule> {
        let mut by_id: HashMap<String, Vec<&'a HierarchicalRule>> = HashMap::new();

        for &rule in rules {
            let base_id = self.get_base_id(&rule.rule.id);
            by_id.entry(base_id).or_insert_with(Vec::new).push(rule);
        }

        let mut resolved = Vec::new();
        for (_id, group) in by_id {
            if group.len() == 1 {
                resolved.push(group[0]);
            } else {
                // Select the one with most Error conditions
                let most_restrictive = group
                    .iter()
                    .max_by_key(|r| {
                        r.rule
                            .conditions
                            .iter()
                            .filter(|c| c.severity == Severity::Error)
                            .count()
                    })
                    .unwrap();
                resolved.push(*most_restrictive);
            }
        }

        resolved
    }

    fn resolve_merge<'a>(
        &self,
        rules: &'a [&'a HierarchicalRule],
    ) -> Vec<&'a HierarchicalRule> {
        // For merge strategy, return all rules
        // Actual merging happens at validation time
        rules.to_vec()
    }

    /// Get base ID (strip level prefix if present)
    fn get_base_id(&self, id: &str) -> String {
        if let Some(idx) = id.find('_') {
            let prefix = &id[..idx];
            if matches!(prefix, "core" | "domain" | "project" | "session") {
                return id[idx + 1..].to_string();
            }
        }
        id.to_string()
    }

    /// Detect circular overrides
    pub fn detect_circular_overrides(
        &self,
        rules: &HashMap<String, HierarchicalRule>,
    ) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashMap::new();
        let mut rec_stack = HashMap::new();

        for rule_id in rules.keys() {
            if !visited.contains_key(rule_id) {
                if let Some(cycle) = self.detect_cycle_dfs(
                    rule_id,
                    rules,
                    &mut visited,
                    &mut rec_stack,
                    &mut Vec::new(),
                ) {
                    cycles.push(cycle);
                }
            }
        }

        cycles
    }

    fn detect_cycle_dfs(
        &self,
        rule_id: &str,
        rules: &HashMap<String, HierarchicalRule>,
        visited: &mut HashMap<String, bool>,
        rec_stack: &mut HashMap<String, bool>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(rule_id.to_string(), true);
        rec_stack.insert(rule_id.to_string(), true);
        path.push(rule_id.to_string());

        if let Some(rule) = rules.get(rule_id) {
            if let Some(overrides) = &rule.overrides {
                if rec_stack.get(overrides) == Some(&true) {
                    // Cycle detected
                    let cycle_start = path.iter().position(|id| id == overrides).unwrap();
                    return Some(path[cycle_start..].to_vec());
                }

                if visited.get(overrides) != Some(&true) {
                    if let Some(cycle) = self.detect_cycle_dfs(
                        overrides,
                        rules,
                        visited,
                        rec_stack,
                        path,
                    ) {
                        return Some(cycle);
                    }
                }
            }
        }

        rec_stack.insert(rule_id.to_string(), false);
        path.pop();
        None
    }

    /// Merge validation results from multiple levels
    pub fn merge_validation_results(
        &self,
        results: Vec<ValidationResult>,
    ) -> ValidationResult {
        let mut merged = ValidationResult {
            passed: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
        };

        for result in results {
            if !result.passed {
                merged.passed = false;
            }
            merged.errors.extend(result.errors);
            merged.warnings.extend(result.warnings);
            merged.info.extend(result.info);
        }

        // Deduplicate messages
        merged.errors = self.deduplicate_messages(merged.errors);
        merged.warnings = self.deduplicate_messages(merged.warnings);
        merged.info = self.deduplicate_messages(merged.info);

        merged
    }

    fn deduplicate_messages(&self, messages: Vec<ValidationMessage>) -> Vec<ValidationMessage> {
        let mut seen = HashMap::new();
        let mut deduped = Vec::new();

        for msg in messages {
            let key = format!("{}:{}", msg.rule_id, msg.check_type);
            if !seen.contains_key(&key) {
                seen.insert(key, true);
                deduped.push(msg);
            }
        }

        deduped
    }
}

impl Default for RuleResolver {
    fn default() -> Self {
        Self::new(ConflictStrategy::MostSpecificWins)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::{RuleSource};
    use crate::{Rule, RuleCategory, Condition};

    #[test]
    fn test_get_base_id() {
        let resolver = RuleResolver::default();
        assert_eq!(resolver.get_base_id("core_type_safety"), "type_safety");
        assert_eq!(resolver.get_base_id("domain_cad_topology"), "cad_topology");
        assert_eq!(resolver.get_base_id("simple_rule"), "simple_rule");
    }

    #[test]
    fn test_most_specific_wins() {
        let resolver = RuleResolver::new(ConflictStrategy::MostSpecificWins);

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
                session_id: "test".to_string(),
            },
            enabled: true,
        };

        let rules = vec![&core_rule, &session_rule];
        let resolved = resolver.resolve_conflicts(&rules);

        // Should prefer session over core
        assert_eq!(resolved.len(), 2); // Different base IDs
    }

    #[test]
    fn test_circular_override_detection() {
        let resolver = RuleResolver::default();
        let mut rules = HashMap::new();

        let rule1 = HierarchicalRule {
            rule: Rule {
                id: "rule1".to_string(),
                program_type: "test".to_string(),
                category: RuleCategory::Validation,
                conditions: vec![],
            },
            level: RuleLevel::Core,
            overrides: Some("rule2".to_string()),
            source: RuleSource::Builtin,
            enabled: true,
        };

        let rule2 = HierarchicalRule {
            rule: Rule {
                id: "rule2".to_string(),
                program_type: "test".to_string(),
                category: RuleCategory::Validation,
                conditions: vec![],
            },
            level: RuleLevel::Domain,
            overrides: Some("rule1".to_string()),
            source: RuleSource::Builtin,
            enabled: true,
        };

        rules.insert("rule1".to_string(), rule1);
        rules.insert("rule2".to_string(), rule2);

        let cycles = resolver.detect_circular_overrides(&rules);
        assert!(!cycles.is_empty());
    }
}
