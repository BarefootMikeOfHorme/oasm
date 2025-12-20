use std::collections::HashMap;
use crate::parser::Instruction;

/// Represents a defined macro in OASM
#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub parameters: Vec<String>,
    pub instructions: Vec<Instruction>,
}

/// Registry for storing and retrieving macros
pub struct MacroRegistry {
    macros: HashMap<String, Macro>,
}

impl MacroRegistry {
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
        }
    }

    pub fn register(&mut self, m: Macro) {
        self.macros.insert(m.name.to_uppercase(), m);
    }

    pub fn get(&self, name: &str) -> Option<&Macro> {
        self.macros.get(&name.to_uppercase())
    }
}

/// Processor responsible for expanding macros before execution
pub struct MacroProcessor {
    registry: MacroRegistry,
}

impl MacroProcessor {
    pub fn new(registry: MacroRegistry) -> Self {
        Self { registry }
    }

    /// Expands a list of instructions, replacing macro calls with their definitions
    pub fn expand(&self, instructions: Vec<Instruction>) -> Vec<Instruction> {
        let mut expanded = Vec::new();

        for instr in instructions {
            if let Some(m) = self.registry.get(&instr.mnemonic) {
                // If the mnemonic matches a macro, expand it
                // For now, we do simple replacement (ignoring params for this basic version)
                for mut macro_instr in m.instructions.clone() {
                    macro_instr.line_number = instr.line_number; // Keep context
                    expanded.push(macro_instr);
                }
            } else {
                expanded.push(instr);
            }
        }

        expanded
    }
}

impl Default for MacroRegistry {
    fn default() -> Self {
        Self::new()
    }
}
