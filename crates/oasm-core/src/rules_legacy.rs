/// Rule system for validation and behavior

use crate::{Rule, RuleCategory, Condition, Severity, RuleEngine};

/// Load rules for a specific program type
pub fn load_rules_for_program(program_type: &str) -> Vec<Rule> {
    match program_type {
        "cad" => vec![
            Rule {
                id: "cad_topology_validation".to_string(),
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
            Rule {
                id: "cad_parameter_range".to_string(),
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
        ],
        "engine" => vec![
            Rule {
                id: "engine_scene_graph_validation".to_string(),
                program_type: "engine".to_string(),
                category: RuleCategory::Validation,
                conditions: vec![
                    Condition {
                        check_type: "no_circular_refs".to_string(),
                        severity: Severity::Error,
                        message: "Scene graph cannot have circular references".to_string(),
                    },
                ],
            },
        ],
        "document" => vec![
            Rule {
                id: "document_structure_validation".to_string(),
                program_type: "document".to_string(),
                category: RuleCategory::Validation,
                conditions: vec![
                    Condition {
                        check_type: "valid_hierarchy".to_string(),
                        severity: Severity::Error,
                        message: "Document hierarchy must be valid".to_string(),
                    },
                ],
            },
        ],
        _ => Vec::new(),
    }
}

/// Initialize rule engine with all rules
pub fn init_rule_engine(engine: &mut RuleEngine) {
    for program_type in &["cad", "engine", "document", "compression", "debug"] {
        for rule in load_rules_for_program(program_type) {
            engine.register_rule(rule);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_cad_rules() {
        let rules = load_rules_for_program("cad");
        assert!(!rules.is_empty());
        assert!(rules.iter().any(|r| r.id == "cad_topology_validation"));
    }

    #[test]
    fn test_init_rule_engine() {
        let mut engine = RuleEngine::new();
        init_rule_engine(&mut engine);
        let cad_rules = engine.get_rules_for_program("cad");
        assert!(!cad_rules.is_empty());
    }
}
