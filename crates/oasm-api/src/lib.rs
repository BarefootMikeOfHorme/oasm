// Core types for OASM API
#[derive(Debug, Clone)]
pub enum ProgramType {
    CAD,
    Engine,
    Document,
    Compression,
    Debug,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Create { object_type: String },
    Define { name: String, value: String },
    Set { property: String, value: String },
    Execute { command: String },
    // Additional instruction types will be added as needed
}

pub struct OasmContext {
    pub rules: Vec<String>,              // TODO: wire into validation
    pub metadata: std::collections::HashMap<String, String>, // TODO: wire into runtime
}

pub fn execute(_instruction: Instruction, _context: &mut OasmContext) -> Result<(), String> {
    Ok(())
}

pub fn parse(_source: &str) -> Result<Vec<Instruction>, String> {
    Ok(vec![])
}

pub fn register_program(_name: &str, _program_type: ProgramType) -> Result<OasmContext, String> {
    Ok(OasmContext { rules: vec![], metadata: std::collections::HashMap::new() })
}
