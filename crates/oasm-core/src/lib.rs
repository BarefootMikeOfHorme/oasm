/// OASM Core - Internal modules, blocks, and scripts
///
/// This crate contains the internal implementation of OASM:
/// - Module system
/// - Block definitions
/// - Rule engine
/// - Instruction parser and executor

pub mod modules;
pub mod blocks;
pub mod rules;          // Hierarchical rule engine (Core→Domain→Project→Session)
pub mod rules_legacy;   // Legacy rule loading functions
pub mod instructions;

// Native modular components
pub mod types;          // Native type system
pub mod context;        // Execution context manager
pub mod parser;         // Native OASM parser
pub mod executor;       // Native instruction executor
pub mod command_blocks; // Command block builder (batching + testing/repair loops)
pub mod validators;     // Validators (type, topology, rules)
pub mod state_evaluator; // Smart state evaluation logic
pub mod macro_processor; // Macro expansion logic
pub mod symbol_table;   // Searchable symbol tracking for debugging
pub mod templates;      // YAML-based template loading and expansion

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Module definition
#[derive(Debug, Clone)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub module_type: ModuleType,
    pub location: PathBuf,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleType {
    Core,
    Extension,
    Optimization,
    Integration,
}

/// Block definition for program-specific functionality
#[derive(Debug, Clone)]
pub struct Block {
    pub id: String,
    pub program_type: String,
    pub instructions: Vec<String>,
    pub rules: Vec<String>,
    pub optimizations: Vec<String>,
}

/// Rule definition (used by hierarchical rule engine in rules/ module)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub program_type: String,
    pub category: RuleCategory,
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleCategory {
    Validation,
    Behavior,
    Constraint,
    Output,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub check_type: String,
    pub severity: Severity,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Module registry
pub struct ModuleRegistry {
    modules: HashMap<String, Module>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn register(&mut self, module: Module) {
        self.modules.insert(module.id.clone(), module);
    }

    pub fn get(&self, id: &str) -> Option<&Module> {
        self.modules.get(id)
    }

    pub fn list(&self) -> Vec<&Module> {
        self.modules.values().collect()
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Block registry
pub struct BlockRegistry {
    blocks: HashMap<String, Block>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    pub fn register(&mut self, block: Block) {
        self.blocks.insert(block.id.clone(), block);
    }

    pub fn get(&self, id: &str) -> Option<&Block> {
        self.blocks.get(id)
    }

    pub fn get_for_program(&self, program_type: &str) -> Vec<&Block> {
        self.blocks
            .values()
            .filter(|b| b.program_type == program_type)
            .collect()
    }
}

impl Default for BlockRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Rule engine
pub struct RuleEngine {
    rules: HashMap<String, Rule>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn register_rule(&mut self, rule: Rule) {
        self.rules.insert(rule.id.clone(), rule);
    }

    pub fn get_rules_for_program(&self, program_type: &str) -> Vec<&Rule> {
        self.rules
            .values()
            .filter(|r| r.program_type == program_type)
            .collect()
    }

    pub fn validate(&self, program_type: &str, _data: &HashMap<String, String>) -> Vec<String> {
        let errors = Vec::new();
        for _rule in self.get_rules_for_program(program_type) {
            // Rule validation logic would go here
        }
        errors
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_registry() {
        let mut registry = ModuleRegistry::new();
        let module = Module {
            id: "test_module".to_string(),
            name: "Test Module".to_string(),
            module_type: ModuleType::Core,
            location: PathBuf::from("/test"),
            capabilities: vec!["cap1".to_string()],
        };
        registry.register(module.clone());
        assert!(registry.get("test_module").is_some());
    }
}
