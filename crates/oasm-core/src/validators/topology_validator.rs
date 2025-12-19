/// Topology validator - validates CAD geometry (manifold, watertight, etc.)

use super::{IssueSeverity, ValidationContext, ValidationIssue, ValidationReport};
use crate::types::Value;

pub struct TopologyValidator {
    strict_mode: bool,
}

impl TopologyValidator {
    pub fn new() -> Self {
        Self {
            strict_mode: false,
        }
    }

    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    pub fn validate(&self, context: &ValidationContext) -> ValidationReport {
        let mut report = ValidationReport::new("topology_validator".to_string());

        // Only validate CAD objects
        if context.program_type != "cad" {
            return report;
        }

        for (obj_id, object) in &context.objects {
            // Check if object has geometry
            if !self.has_geometry(&object.properties) {
                continue;
            }

            // Validate manifold
            if let Err(msg) = self.check_manifold(&object.properties) {
                report.add_issue(ValidationIssue {
                    severity: IssueSeverity::Error,
                    code: "NOT_MANIFOLD".to_string(),
                    message: format!("Object '{}' is not manifold: {}", obj_id, msg),
                    location: Some(super::IssueLocation {
                        file: None,
                        line: None,
                        column: None,
                        object_id: Some(obj_id.clone()),
                    }),
                    suggestion: Some("Ensure all edges are connected to exactly two faces".to_string()),
                });
            }

            // Validate watertight (closed)
            if let Err(msg) = self.check_watertight(&object.properties) {
                report.add_issue(ValidationIssue {
                    severity: IssueSeverity::Error,
                    code: "NOT_WATERTIGHT".to_string(),
                    message: format!("Object '{}' is not watertight: {}", obj_id, msg),
                    location: Some(super::IssueLocation {
                        file: None,
                        line: None,
                        column: None,
                        object_id: Some(obj_id.clone()),
                    }),
                    suggestion: Some("Ensure all edges form closed loops".to_string()),
                });
            }

            // Check for self-intersections
            if let Err(msg) = self.check_no_self_intersections(&object.properties) {
                report.add_issue(ValidationIssue {
                    severity: IssueSeverity::Error,
                    code: "SELF_INTERSECTION".to_string(),
                    message: format!("Object '{}' has self-intersections: {}", obj_id, msg),
                    location: Some(super::IssueLocation {
                        file: None,
                        line: None,
                        column: None,
                        object_id: Some(obj_id.clone()),
                    }),
                    suggestion: Some("Remove overlapping geometry".to_string()),
                });
            }

            // Check face normals
            if self.strict_mode {
                if let Err(msg) = self.check_face_normals(&object.properties) {
                    report.add_issue(ValidationIssue {
                        severity: IssueSeverity::Warning,
                        code: "INCONSISTENT_NORMALS".to_string(),
                        message: format!("Object '{}' has inconsistent normals: {}", obj_id, msg),
                        location: Some(super::IssueLocation {
                            file: None,
                            line: None,
                            column: None,
                            object_id: Some(obj_id.clone()),
                        }),
                        suggestion: Some("Recalculate face normals".to_string()),
                    });
                }
            }

            // Check degenerate geometry
            if let Err(msg) = self.check_no_degenerate_faces(&object.properties) {
                report.add_issue(ValidationIssue {
                    severity: IssueSeverity::Warning,
                    code: "DEGENERATE_GEOMETRY".to_string(),
                    message: format!("Object '{}' has degenerate geometry: {}", obj_id, msg),
                    location: Some(super::IssueLocation {
                        file: None,
                        line: None,
                        column: None,
                        object_id: Some(obj_id.clone()),
                    }),
                    suggestion: Some("Remove zero-area faces".to_string()),
                });
            }
        }

        report
    }

    fn has_geometry(&self, _properties: &std::collections::HashMap<String, Value>) -> bool {
        // TODO: Check if properties contain mesh/geometry data
        // For now, assume all objects have geometry
        true
    }

    fn check_manifold(&self, properties: &std::collections::HashMap<String, Value>) -> Result<(), String> {
        // TODO: Implement actual manifold checking
        // A mesh is manifold if every edge is connected to exactly 2 faces
        // For now, placeholder check
        if properties.contains_key("non_manifold_edges") {
            return Err("Mesh has non-manifold edges".to_string());
        }
        Ok(())
    }

    fn check_watertight(&self, properties: &std::collections::HashMap<String, Value>) -> Result<(), String> {
        // TODO: Implement actual watertight checking
        // A mesh is watertight if all edges form closed loops
        if properties.contains_key("open_edges") {
            return Err("Mesh has open edges".to_string());
        }
        Ok(())
    }

    fn check_no_self_intersections(&self, properties: &std::collections::HashMap<String, Value>) -> Result<(), String> {
        // TODO: Implement actual self-intersection detection
        // Check if any faces intersect each other
        if properties.contains_key("self_intersecting") {
            return Err("Mesh has self-intersecting faces".to_string());
        }
        Ok(())
    }

    fn check_face_normals(&self, properties: &std::collections::HashMap<String, Value>) -> Result<(), String> {
        // TODO: Implement normal consistency checking
        // Check if all face normals point outward consistently
        if properties.contains_key("flipped_normals") {
            return Err("Mesh has inconsistent face normals".to_string());
        }
        Ok(())
    }

    fn check_no_degenerate_faces(&self, properties: &std::collections::HashMap<String, Value>) -> Result<(), String> {
        // TODO: Implement degenerate geometry detection
        // Check for zero-area faces, duplicate vertices, etc.
        if properties.contains_key("degenerate_faces") {
            return Err("Mesh has degenerate faces".to_string());
        }
        Ok(())
    }
}

impl Default for TopologyValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Object;
    use chrono::Utc;

    #[test]
    fn test_non_cad_objects_pass() {
        let validator = TopologyValidator::new();
        let context = ValidationContext::new("engine".to_string());

        let report = validator.validate(&context);
        assert!(report.passed);
    }

    #[test]
    fn test_cad_object_validation() {
        let validator = TopologyValidator::new();
        let mut context = ValidationContext::new("cad".to_string());

        let mut object = Object {
            id: "test_mesh".to_string(),
            object_type: "mesh".to_string(),
            properties: std::collections::HashMap::new(),
            created: Utc::now(),
        };

        // Add a property indicating non-manifold edges
        object.properties.insert(
            "non_manifold_edges".to_string(),
            Value::Bool(true),
        );

        context.objects.insert("test_mesh".to_string(), object);

        let report = validator.validate(&context);
        assert!(!report.passed);
        assert!(report.error_count() > 0);
    }
}
