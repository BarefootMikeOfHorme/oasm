/// OASM Validators - Type, Topology, and Rules validation

pub mod type_validator;
pub mod topology_validator;
pub mod rules_validator;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub passed: bool,
    pub validator: String,
    pub issues: Vec<ValidationIssue>,
    pub metadata: HashMap<String, String>,
}

/// Validation issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub severity: IssueSeverity,
    pub code: String,
    pub message: String,
    pub location: Option<IssueLocation>,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
}

/// Location of a validation issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueLocation {
    pub file: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub object_id: Option<String>,
}

impl ValidationReport {
    pub fn new(validator: String) -> Self {
        Self {
            passed: true,
            validator,
            issues: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_issue(&mut self, issue: ValidationIssue) {
        if issue.severity == IssueSeverity::Error {
            self.passed = false;
        }
        self.issues.push(issue);
    }

    pub fn add_error(&mut self, code: String, message: String) {
        self.add_issue(ValidationIssue {
            severity: IssueSeverity::Error,
            code,
            message,
            location: None,
            suggestion: None,
        });
    }

    pub fn add_warning(&mut self, code: String, message: String) {
        self.add_issue(ValidationIssue {
            severity: IssueSeverity::Warning,
            code,
            message,
            location: None,
            suggestion: None,
        });
    }

    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == IssueSeverity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == IssueSeverity::Warning)
            .count()
    }

    pub fn merge(&mut self, other: ValidationReport) {
        if !other.passed {
            self.passed = false;
        }
        self.issues.extend(other.issues);
        self.metadata.extend(other.metadata);
    }
}

/// Combined validator that runs all validators
pub struct CombinedValidator {
    pub type_validator: type_validator::TypeValidator,
    pub topology_validator: topology_validator::TopologyValidator,
    pub rules_validator: rules_validator::RulesValidator,
}

impl CombinedValidator {
    pub fn new() -> Self {
        Self {
            type_validator: type_validator::TypeValidator::new(),
            topology_validator: topology_validator::TopologyValidator::new(),
            rules_validator: rules_validator::RulesValidator::new(),
        }
    }

    /// Run all validators and combine results
    pub fn validate_all(&self, context: &ValidationContext) -> ValidationReport {
        let mut combined = ValidationReport::new("combined".to_string());

        // Run type validation
        let type_report = self.type_validator.validate(context);
        combined.merge(type_report);

        // Run topology validation (only for CAD objects)
        if context.program_type == "cad" {
            let topology_report = self.topology_validator.validate(context);
            combined.merge(topology_report);
        }

        // Run rules validation
        let rules_report = self.rules_validator.validate(context);
        combined.merge(rules_report);

        combined
    }
}

impl Default for CombinedValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation context - data passed to validators
#[derive(Debug, Clone)]
pub struct ValidationContext {
    pub program_type: String,
    pub objects: HashMap<String, crate::context::Object>,
    pub variables: HashMap<String, crate::context::Variable>,
    pub properties: HashMap<String, String>,
}

impl ValidationContext {
    pub fn new(program_type: String) -> Self {
        Self {
            program_type,
            objects: HashMap::new(),
            variables: HashMap::new(),
            properties: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_report() {
        let mut report = ValidationReport::new("test".to_string());
        assert!(report.passed);
        assert_eq!(report.error_count(), 0);

        report.add_error("E001".to_string(), "Test error".to_string());
        assert!(!report.passed);
        assert_eq!(report.error_count(), 1);

        report.add_warning("W001".to_string(), "Test warning".to_string());
        assert_eq!(report.warning_count(), 1);
    }

    #[test]
    fn test_merge_reports() {
        let mut report1 = ValidationReport::new("test1".to_string());
        report1.add_error("E001".to_string(), "Error 1".to_string());

        let mut report2 = ValidationReport::new("test2".to_string());
        report2.add_warning("W001".to_string(), "Warning 1".to_string());

        report1.merge(report2);
        assert_eq!(report1.error_count(), 1);
        assert_eq!(report1.warning_count(), 1);
    }
}
