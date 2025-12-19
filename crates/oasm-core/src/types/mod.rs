/// OASM Native Type System
///
/// Defines OASM's native types: primitives, composites, geometric, objects

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OASM native type system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OasmType {
    // Primitive types
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Char,
    String,

    // Composite types
    Array {
        element_type: Box<OasmType>,
        size: usize,
    },
    Struct {
        name: String,
        fields: Vec<Field>,
    },
    Enum {
        name: String,
        variants: Vec<Variant>,
    },

    // Geometric types (CAD-specific)
    Vector2,
    Vector3,
    Vector4,
    Matrix3x3,
    Matrix4x4,
    BoundingBox,
    Mesh,

    // Object types (runtime objects)
    Object {
        object_type: String,
    },

    // Special types
    Void,
    Unknown,
}

/// Field in a struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: OasmType,
}

/// Variant in an enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    pub name: String,
    pub fields: Option<Vec<Field>>,
}

/// Runtime value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Char(char),
    String(String),
    Array(Vec<Value>),
    Struct {
        name: String,
        fields: HashMap<String, Value>,
    },
    Enum {
        name: String,
        variant: String,
        fields: Option<HashMap<String, Value>>,
    },
    Vector2([f64; 2]),
    Vector3([f64; 3]),
    Vector4([f64; 4]),
    Matrix3x3([[f64; 3]; 3]),
    Matrix4x4([[f64; 4]; 4]),
    BoundingBox {
        min: [f64; 3],
        max: [f64; 3],
    },
    Mesh {
        vertices: Vec<[f64; 3]>,
        faces: Vec<Vec<usize>>,
    },
    Object {
        id: String,
        object_type: String,
        properties: HashMap<String, Value>,
    },
    Void,
}

/// Type checker trait
pub trait TypeChecker {
    /// Infer the type of a value
    fn infer_type(&self, value: &Value) -> OasmType;

    /// Check if an assignment is valid
    fn check_assignment(&self, target: &OasmType, value: &OasmType) -> Result<(), TypeError>;

    /// Validate an operation and return result type
    fn validate_operation(
        &self,
        op: &Operation,
        operands: &[OasmType],
    ) -> Result<OasmType, TypeError>;

    /// Check if a type can be cast to another
    fn can_cast(&self, from: &OasmType, to: &OasmType) -> bool;
}

/// Operation types
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    // Comparison
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,

    // Logical
    And,
    Or,
    Not,

    // Geometric
    Dot,
    Cross,
    MatrixMultiply,

    // Object
    PropertyAccess,
    MethodCall,
}

/// Type errors
#[derive(Debug, Clone)]
pub enum TypeError {
    TypeMismatch {
        expected: OasmType,
        found: OasmType,
    },
    UndefinedVariable(String),
    UndefinedField {
        struct_name: String,
        field_name: String,
    },
    InvalidOperation {
        op: Operation,
        operands: Vec<OasmType>,
    },
    InvalidCast {
        from: OasmType,
        to: OasmType,
    },
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TypeError::TypeMismatch { expected, found } => {
                write!(f, "Type mismatch: expected {:?}, found {:?}", expected, found)
            }
            TypeError::UndefinedVariable(name) => {
                write!(f, "Undefined variable: {}", name)
            }
            TypeError::UndefinedField { struct_name, field_name } => {
                write!(f, "Undefined field '{}' in struct '{}'", field_name, struct_name)
            }
            TypeError::InvalidOperation { op, operands } => {
                write!(f, "Invalid operation {:?} for operands {:?}", op, operands)
            }
            TypeError::InvalidCast { from, to } => {
                write!(f, "Invalid cast from {:?} to {:?}", from, to)
            }
        }
    }
}

impl std::error::Error for TypeError {}

/// Native type checker implementation
pub struct NativeTypeChecker;

impl TypeChecker for NativeTypeChecker {
    fn infer_type(&self, value: &Value) -> OasmType {
        match value {
            Value::U8(_) => OasmType::U8,
            Value::U16(_) => OasmType::U16,
            Value::U32(_) => OasmType::U32,
            Value::U64(_) => OasmType::U64,
            Value::I8(_) => OasmType::I8,
            Value::I16(_) => OasmType::I16,
            Value::I32(_) => OasmType::I32,
            Value::I64(_) => OasmType::I64,
            Value::F32(_) => OasmType::F32,
            Value::F64(_) => OasmType::F64,
            Value::Bool(_) => OasmType::Bool,
            Value::Char(_) => OasmType::Char,
            Value::String(_) => OasmType::String,
            Value::Array(arr) => {
                if arr.is_empty() {
                    OasmType::Array {
                        element_type: Box::new(OasmType::Unknown),
                        size: 0,
                    }
                } else {
                    OasmType::Array {
                        element_type: Box::new(self.infer_type(&arr[0])),
                        size: arr.len(),
                    }
                }
            }
            Value::Struct { name, .. } => OasmType::Struct {
                name: name.clone(),
                fields: vec![], // Would need full struct def
            },
            Value::Enum { name, .. } => OasmType::Enum {
                name: name.clone(),
                variants: vec![], // Would need full enum def
            },
            Value::Vector2(_) => OasmType::Vector2,
            Value::Vector3(_) => OasmType::Vector3,
            Value::Vector4(_) => OasmType::Vector4,
            Value::Matrix3x3(_) => OasmType::Matrix3x3,
            Value::Matrix4x4(_) => OasmType::Matrix4x4,
            Value::BoundingBox { .. } => OasmType::BoundingBox,
            Value::Mesh { .. } => OasmType::Mesh,
            Value::Object { object_type, .. } => OasmType::Object {
                object_type: object_type.clone(),
            },
            Value::Void => OasmType::Void,
        }
    }

    fn check_assignment(&self, target: &OasmType, value: &OasmType) -> Result<(), TypeError> {
        if target == value {
            Ok(())
        } else if self.can_cast(value, target) {
            Ok(())
        } else {
            Err(TypeError::TypeMismatch {
                expected: target.clone(),
                found: value.clone(),
            })
        }
    }

    fn validate_operation(
        &self,
        op: &Operation,
        operands: &[OasmType],
    ) -> Result<OasmType, TypeError> {
        match op {
            Operation::Add | Operation::Subtract | Operation::Multiply | Operation::Divide | Operation::Modulo => {
                // Numeric operations
                if operands.len() != 2 {
                    return Err(TypeError::InvalidOperation {
                        op: op.clone(),
                        operands: operands.to_vec(),
                    });
                }

                match (&operands[0], &operands[1]) {
                    (OasmType::F64, OasmType::F64) => Ok(OasmType::F64),
                    (OasmType::F32, OasmType::F32) => Ok(OasmType::F32),
                    (OasmType::I64, OasmType::I64) => Ok(OasmType::I64),
                    (OasmType::I32, OasmType::I32) => Ok(OasmType::I32),
                    (OasmType::U64, OasmType::U64) => Ok(OasmType::U64),
                    (OasmType::U32, OasmType::U32) => Ok(OasmType::U32),
                    _ => Err(TypeError::InvalidOperation {
                        op: op.clone(),
                        operands: operands.to_vec(),
                    }),
                }
            }
            Operation::Equal
            | Operation::NotEqual
            | Operation::LessThan
            | Operation::LessOrEqual
            | Operation::GreaterThan
            | Operation::GreaterOrEqual => {
                // Comparison operations return bool
                if operands.len() != 2 {
                    return Err(TypeError::InvalidOperation {
                        op: op.clone(),
                        operands: operands.to_vec(),
                    });
                }
                Ok(OasmType::Bool)
            }
            Operation::And | Operation::Or => {
                // Logical operations on bools
                if operands.len() != 2 {
                    return Err(TypeError::InvalidOperation {
                        op: op.clone(),
                        operands: operands.to_vec(),
                    });
                }
                if operands[0] == OasmType::Bool && operands[1] == OasmType::Bool {
                    Ok(OasmType::Bool)
                } else {
                    Err(TypeError::InvalidOperation {
                        op: op.clone(),
                        operands: operands.to_vec(),
                    })
                }
            }
            Operation::Not => {
                // Logical not on bool
                if operands.len() != 1 {
                    return Err(TypeError::InvalidOperation {
                        op: op.clone(),
                        operands: operands.to_vec(),
                    });
                }
                if operands[0] == OasmType::Bool {
                    Ok(OasmType::Bool)
                } else {
                    Err(TypeError::InvalidOperation {
                        op: op.clone(),
                        operands: operands.to_vec(),
                    })
                }
            }
            Operation::Dot => {
                // Dot product: Vector3 x Vector3 → F64
                if operands.len() != 2 {
                    return Err(TypeError::InvalidOperation {
                        op: op.clone(),
                        operands: operands.to_vec(),
                    });
                }
                match (&operands[0], &operands[1]) {
                    (OasmType::Vector3, OasmType::Vector3) => Ok(OasmType::F64),
                    _ => Err(TypeError::InvalidOperation {
                        op: op.clone(),
                        operands: operands.to_vec(),
                    }),
                }
            }
            Operation::Cross => {
                // Cross product: Vector3 x Vector3 → Vector3
                if operands.len() != 2 {
                    return Err(TypeError::InvalidOperation {
                        op: op.clone(),
                        operands: operands.to_vec(),
                    });
                }
                match (&operands[0], &operands[1]) {
                    (OasmType::Vector3, OasmType::Vector3) => Ok(OasmType::Vector3),
                    _ => Err(TypeError::InvalidOperation {
                        op: op.clone(),
                        operands: operands.to_vec(),
                    }),
                }
            }
            Operation::MatrixMultiply => {
                // Matrix4x4 x Vector4 → Vector4
                if operands.len() != 2 {
                    return Err(TypeError::InvalidOperation {
                        op: op.clone(),
                        operands: operands.to_vec(),
                    });
                }
                match (&operands[0], &operands[1]) {
                    (OasmType::Matrix4x4, OasmType::Vector4) => Ok(OasmType::Vector4),
                    (OasmType::Matrix4x4, OasmType::Matrix4x4) => Ok(OasmType::Matrix4x4),
                    _ => Err(TypeError::InvalidOperation {
                        op: op.clone(),
                        operands: operands.to_vec(),
                    }),
                }
            }
            Operation::PropertyAccess | Operation::MethodCall => {
                // Would need full object definition
                Ok(OasmType::Unknown)
            }
        }
    }

    fn can_cast(&self, from: &OasmType, to: &OasmType) -> bool {
        // Primitive numeric casts
        match (from, to) {
            // Same type
            (a, b) if a == b => true,

            // Widening integer casts
            (OasmType::U8, OasmType::U16 | OasmType::U32 | OasmType::U64) => true,
            (OasmType::U16, OasmType::U32 | OasmType::U64) => true,
            (OasmType::U32, OasmType::U64) => true,

            (OasmType::I8, OasmType::I16 | OasmType::I32 | OasmType::I64) => true,
            (OasmType::I16, OasmType::I32 | OasmType::I64) => true,
            (OasmType::I32, OasmType::I64) => true,

            // Float casts
            (OasmType::F32, OasmType::F64) => true,

            // Integer to float
            (OasmType::U8 | OasmType::U16 | OasmType::U32 | OasmType::I8 | OasmType::I16 | OasmType::I32, OasmType::F32 | OasmType::F64) => true,

            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_primitive_types() {
        let checker = NativeTypeChecker;

        assert_eq!(checker.infer_type(&Value::U32(42)), OasmType::U32);
        assert_eq!(checker.infer_type(&Value::F64(3.14)), OasmType::F64);
        assert_eq!(checker.infer_type(&Value::Bool(true)), OasmType::Bool);
        assert_eq!(
            checker.infer_type(&Value::String("hello".to_string())),
            OasmType::String
        );
    }

    #[test]
    fn test_check_assignment() {
        let checker = NativeTypeChecker;

        // Same type
        assert!(checker.check_assignment(&OasmType::U32, &OasmType::U32).is_ok());

        // Valid cast
        assert!(checker.check_assignment(&OasmType::U32, &OasmType::U8).is_ok());

        // Invalid cast
        assert!(checker.check_assignment(&OasmType::Bool, &OasmType::U32).is_err());
    }

    #[test]
    fn test_validate_numeric_operation() {
        let checker = NativeTypeChecker;

        let result = checker.validate_operation(&Operation::Add, &[OasmType::F64, OasmType::F64]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), OasmType::F64);
    }

    #[test]
    fn test_validate_comparison_operation() {
        let checker = NativeTypeChecker;

        let result =
            checker.validate_operation(&Operation::LessThan, &[OasmType::I32, OasmType::I32]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), OasmType::Bool);
    }

    #[test]
    fn test_validate_vector_operations() {
        let checker = NativeTypeChecker;

        // Dot product
        let result =
            checker.validate_operation(&Operation::Dot, &[OasmType::Vector3, OasmType::Vector3]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), OasmType::F64);

        // Cross product
        let result =
            checker.validate_operation(&Operation::Cross, &[OasmType::Vector3, OasmType::Vector3]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), OasmType::Vector3);
    }

    #[test]
    fn test_can_cast() {
        let checker = NativeTypeChecker;

        // Widening casts
        assert!(checker.can_cast(&OasmType::U8, &OasmType::U32));
        assert!(checker.can_cast(&OasmType::I16, &OasmType::I64));
        assert!(checker.can_cast(&OasmType::F32, &OasmType::F64));

        // Integer to float
        assert!(checker.can_cast(&OasmType::U32, &OasmType::F64));

        // Invalid casts
        assert!(!checker.can_cast(&OasmType::Bool, &OasmType::U32));
        assert!(!checker.can_cast(&OasmType::String, &OasmType::F64));
    }
}
