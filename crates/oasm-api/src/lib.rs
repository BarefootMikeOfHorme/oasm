/// OASM API - External interface for programs running WITH OASM
///
/// This crate provides the public API for external programs (CAD editors,
/// engine editors, word processors, etc.) to integrate with the OASM
/// assembly module system.

use std::collections::HashMap;

/// Program types supported by OASM
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramType {
    CAD,
    Engine,
    Document,
    Compression,
    Debug,
    Custom(String),
}

/// OASM Context for external programs
pub struct OasmContext {
    program_type: ProgramType,
    capabilities: Vec<String>,
    rules: Vec<String>,
    metadata: HashMap<String, String>,
}

impl OasmContext {
    /// Create a new OASM context for a program
    pub fn new(program_type: ProgramType) -> Self {
        Self {
            program_type,
            capabilities: Vec::new(),
            rules: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Load rules for this program type
    pub fn load_rules(&mut self) -> Result<(), String> {
        // Rules will be loaded from oasm-core
        Ok(())
    }

    /// Enable a capability
    pub fn enable_capability(&mut self, cap: &str) {
        if !self.capabilities.contains(&cap.to_string()) {
            self.capabilities.push(cap.to_string());
        }
    }

    /// Check if a capability is enabled
    pub fn has_capability(&self, cap: &str) -> bool {
        self.capabilities.contains(&cap.to_string())
    }

    /// Get program type
    pub fn program_type(&self) -> &ProgramType {
        &self.program_type
    }
}

/// OASM Instruction - Assembly-like operations
#[derive(Debug, Clone)]
pub enum Instruction {
    /// CREATE object [parameters]
    Create { object_type: String, params: HashMap<String, Value> },

    /// DEFINE name [settings]
    Define { name: String, settings: HashMap<String, Value> },

    /// SET parameter = value
    Set { parameter: String, value: Value },

    /// EXTRUDE direction, distance
    Extrude { direction: String, distance: f64 },

    /// MOVE object, position
    Move { object: String, position: [f64; 3] },

    /// ROTATE object, rotation
    Rotate { object: String, rotation: [f64; 3] },

    /// SCALE object, factor
    Scale { object: String, factor: f64 },

    /// VALIDATE check_type
    Validate { check_type: String },

    /// SCAN metric
    Scan { metric: String },

    /// EXPORT format, path
    Export { format: String, path: String },
}

/// Value types for parameters
#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
}

/// Execute an OASM instruction
pub fn execute(instruction: Instruction, context: &mut OasmContext) -> Result<(), String> {
    // Execution will be handled by oasm-core
    Ok(())
}

/// Parse OASM assembly text into instructions
pub fn parse(source: &str) -> Result<Vec<Instruction>, String> {
    // Parsing will be handled by oasm-core
    Ok(Vec::new())
}

/// Register a program with OASM
pub fn register_program(name: &str, program_type: ProgramType) -> Result<OasmContext, String> {
    Ok(OasmContext::new(program_type))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let ctx = OasmContext::new(ProgramType::CAD);
        assert_eq!(*ctx.program_type(), ProgramType::CAD);
    }

    #[test]
    fn test_capability_management() {
        let mut ctx = OasmContext::new(ProgramType::CAD);
        ctx.enable_capability("file_watch");
        assert!(ctx.has_capability("file_watch"));
    }
}
