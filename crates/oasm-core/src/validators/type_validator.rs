/// Type validator - validates type safety and correctness

use super::{IssueSeverity, ValidationContext, ValidationIssue, ValidationReport};
use crate::types::{NativeTypeChecker, OasmType, TypeChecker, TypeError};

pub struct TypeValidator {
    type_checker: NativeTypeChecker,
}

impl TypeValidator {
    pub fn new() -> Self {
        Self {
            type_checker: NativeTypeChecker,
        }
    }

    pub fn validate(&self, context: &ValidationContext) -> ValidationReport {
        let mut report = ValidationReport::new("type_validator".to_string());

        // Validate all variables
        for (name, variable) in &context.variables {
            if let Some(value) = &variable.value {
                // Infer the actual type from the value
                let inferred_type = self.type_checker.infer_type(value);

                // Check if it matches the declared type
                if let Err(type_error) = self
                    .type_checker
                    .check_assignment(&variable.var_type, &inferred_type)
                {
                    report.add_issue(ValidationIssue {
                        severity: IssueSeverity::Error,
                        code: "TYPE_MISMATCH".to_string(),
                        message: format!(
                            "Variable '{}' has type mismatch: {}",
                            name, type_error
                        ),
                        location: None,
                        suggestion: Some(format!(
                            "Ensure the value matches the declared type '{:?}'",
                            variable.var_type
                        )),
                    });
                }
            }
        }

        // Validate object properties
        for (obj_id, object) in &context.objects {
            for (prop_name, prop_value) in &object.properties {
                // Infer type from value
                let inferred_type = self.type_checker.infer_type(prop_value);

                // Check if the type is valid for this object type
                if let Err(e) = self.validate_property_type(&object.object_type, prop_name, &inferred_type) {
                    report.add_issue(ValidationIssue {
                        severity: IssueSeverity::Warning,
                        code: "INVALID_PROPERTY_TYPE".to_string(),
                        message: format!(
                            "Object '{}' property '{}' has unexpected type: {}",
                            obj_id, prop_name, e
                        ),
                        location: Some(super::IssueLocation {
                            file: None,
                            line: None,
                            column: None,
                            object_id: Some(obj_id.clone()),
                        }),
                        suggestion: None,
                    });
                }
            }
        }

        report
    }

    fn validate_property_type(
        &self,
        _object_type: &str,
        _prop_name: &str,
        _prop_type: &OasmType,
    ) -> Result<(), TypeError> {
        // TODO: Implement property type validation based on object schemas
        // For now, accept all types
        Ok(())
    }
}

impl Default for TypeValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Variable;
    use crate::types::{OasmType, Value};

    #[test]
    fn test_type_mismatch_detection() {
        let validator = TypeValidator::new();
        let mut context = ValidationContext::new("test".to_string());

        // Add a variable with type mismatch
        let var = Variable {
            name: "test_var".to_string(),
            var_type: OasmType::U32,
            value: Some(Value::String("not a number".to_string())),
            mutable: true,
        };
        context.variables.insert("test_var".to_string(), var);

        let report = validator.validate(&context);
        assert!(!report.passed);
        assert_eq!(report.error_count(), 1);
    }

    #[test]
    fn test_valid_types() {
        let validator = TypeValidator::new();
        let mut context = ValidationContext::new("test".to_string());

        // Add a variable with correct type
        let var = Variable {
            name: "test_var".to_string(),
            var_type: OasmType::U32,
            value: Some(Value::U32(42)),
            mutable: true,
        };
        context.variables.insert("test_var".to_string(), var);

        let report = validator.validate(&context);
        assert!(report.passed);
        assert_eq!(report.error_count(), 0);
    }
}
