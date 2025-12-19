/// Rule hierarchy management and built-in rules for each level

use super::{HierarchicalRule, RuleLevel, RuleSource};
use crate::{Condition, Rule, RuleCategory, Severity};

/// Core-level rules (system-wide, cannot be overridden by default)
pub fn get_core_rules() -> Vec<HierarchicalRule> {
    vec![
        HierarchicalRule {
            rule: Rule {
                id: "core_type_safety".to_string(),
                program_type: "all".to_string(),
                category: RuleCategory::Validation,
                conditions: vec![
                    Condition {
                        check_type: "type_mismatch".to_string(),
                        severity: Severity::Error,
                        message: "Type mismatch detected".to_string(),
                    },
                    Condition {
                        check_type: "invalid_cast".to_string(),
                        severity: Severity::Error,
                        message: "Invalid type cast".to_string(),
                    },
                ],
            },
            level: RuleLevel::Core,
            overrides: None,
            source: RuleSource::Builtin,
            enabled: true,
        },
        HierarchicalRule {
            rule: Rule {
                id: "core_memory_safety".to_string(),
                program_type: "all".to_string(),
                category: RuleCategory::Validation,
                conditions: vec![
                    Condition {
                        check_type: "null_reference".to_string(),
                        severity: Severity::Error,
                        message: "Null reference detected".to_string(),
                    },
                    Condition {
                        check_type: "out_of_bounds".to_string(),
                        severity: Severity::Error,
                        message: "Array index out of bounds".to_string(),
                    },
                ],
            },
            level: RuleLevel::Core,
            overrides: None,
            source: RuleSource::Builtin,
            enabled: true,
        },
        HierarchicalRule {
            rule: Rule {
                id: "core_resource_limits".to_string(),
                program_type: "all".to_string(),
                category: RuleCategory::Constraint,
                conditions: vec![
                    Condition {
                        check_type: "max_memory".to_string(),
                        severity: Severity::Warning,
                        message: "Approaching memory limit".to_string(),
                    },
                    Condition {
                        check_type: "max_execution_time".to_string(),
                        severity: Severity::Warning,
                        message: "Execution time limit exceeded".to_string(),
                    },
                ],
            },
            level: RuleLevel::Core,
            overrides: None,
            source: RuleSource::Builtin,
            enabled: true,
        },
    ]
}

/// Domain-level rules (program-type specific)
pub fn get_domain_rules() -> Vec<HierarchicalRule> {
    vec![
        // CAD domain rules
        HierarchicalRule {
            rule: Rule {
                id: "domain_cad_topology".to_string(),
                program_type: "cad".to_string(),
                category: RuleCategory::Validation,
                conditions: vec![
                    Condition {
                        check_type: "edges_connected".to_string(),
                        severity: Severity::Error,
                        message: "All edges must be connected".to_string(),
                    },
                    Condition {
                        check_type: "faces_closed".to_string(),
                        severity: Severity::Error,
                        message: "All faces must be closed".to_string(),
                    },
                    Condition {
                        check_type: "no_self_intersections".to_string(),
                        severity: Severity::Error,
                        message: "Geometry cannot self-intersect".to_string(),
                    },
                ],
            },
            level: RuleLevel::Domain,
            overrides: None,
            source: RuleSource::Builtin,
            enabled: true,
        },
        HierarchicalRule {
            rule: Rule {
                id: "domain_cad_manifold".to_string(),
                program_type: "cad".to_string(),
                category: RuleCategory::Validation,
                conditions: vec![
                    Condition {
                        check_type: "is_manifold".to_string(),
                        severity: Severity::Error,
                        message: "Mesh must be manifold (watertight)".to_string(),
                    },
                ],
            },
            level: RuleLevel::Domain,
            overrides: None,
            source: RuleSource::Builtin,
            enabled: true,
        },
        HierarchicalRule {
            rule: Rule {
                id: "domain_cad_parameter_range".to_string(),
                program_type: "cad".to_string(),
                category: RuleCategory::Constraint,
                conditions: vec![
                    Condition {
                        check_type: "parameters_in_bounds".to_string(),
                        severity: Severity::Warning,
                        message: "Parameter out of recommended range".to_string(),
                    },
                ],
            },
            level: RuleLevel::Domain,
            overrides: None,
            source: RuleSource::Builtin,
            enabled: true,
        },
        // Engine domain rules
        HierarchicalRule {
            rule: Rule {
                id: "domain_engine_scene_graph".to_string(),
                program_type: "engine".to_string(),
                category: RuleCategory::Validation,
                conditions: vec![
                    Condition {
                        check_type: "no_circular_refs".to_string(),
                        severity: Severity::Error,
                        message: "Scene graph cannot have circular references".to_string(),
                    },
                    Condition {
                        check_type: "valid_transforms".to_string(),
                        severity: Severity::Error,
                        message: "All transforms must be valid (no NaN/Inf)".to_string(),
                    },
                ],
            },
            level: RuleLevel::Domain,
            overrides: None,
            source: RuleSource::Builtin,
            enabled: true,
        },
        HierarchicalRule {
            rule: Rule {
                id: "domain_engine_render_limits".to_string(),
                program_type: "engine".to_string(),
                category: RuleCategory::Constraint,
                conditions: vec![
                    Condition {
                        check_type: "max_draw_calls".to_string(),
                        severity: Severity::Warning,
                        message: "Exceeding recommended draw call limit".to_string(),
                    },
                    Condition {
                        check_type: "max_vertex_count".to_string(),
                        severity: Severity::Warning,
                        message: "Exceeding recommended vertex count".to_string(),
                    },
                ],
            },
            level: RuleLevel::Domain,
            overrides: None,
            source: RuleSource::Builtin,
            enabled: true,
        },
        // Document domain rules
        HierarchicalRule {
            rule: Rule {
                id: "domain_document_structure".to_string(),
                program_type: "document".to_string(),
                category: RuleCategory::Validation,
                conditions: vec![
                    Condition {
                        check_type: "valid_hierarchy".to_string(),
                        severity: Severity::Error,
                        message: "Document hierarchy must be valid".to_string(),
                    },
                    Condition {
                        check_type: "no_orphaned_elements".to_string(),
                        severity: Severity::Warning,
                        message: "Document contains orphaned elements".to_string(),
                    },
                ],
            },
            level: RuleLevel::Domain,
            overrides: None,
            source: RuleSource::Builtin,
            enabled: true,
        },
    ]
}

/// Initialize engine with all built-in rules (Core + Domain)
pub fn load_builtin_rules() -> Vec<HierarchicalRule> {
    let mut rules = Vec::new();
    rules.extend(get_core_rules());
    rules.extend(get_domain_rules());
    rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_rules_count() {
        let core_rules = get_core_rules();
        assert_eq!(core_rules.len(), 3);
        assert!(core_rules.iter().all(|r| r.level == RuleLevel::Core));
    }

    #[test]
    fn test_domain_rules_count() {
        let domain_rules = get_domain_rules();
        assert!(domain_rules.len() >= 6);
        assert!(domain_rules.iter().all(|r| r.level == RuleLevel::Domain));
    }

    #[test]
    fn test_all_builtin_rules() {
        let all_rules = load_builtin_rules();
        assert!(all_rules.len() >= 9);
    }
}
