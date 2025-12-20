use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::OasmType;

/// Metadata for a single symbol (object or variable)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolMetadata {
    pub name: String,
    pub symbol_type: SymbolType,
    pub data_type: OasmType,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub source_line: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SymbolType {
    Object,
    Variable,
    Macro,
    Constant,
}

/// A centralized table for tracking project-wide symbols
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SymbolTable {
    symbols: HashMap<String, SymbolMetadata>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, metadata: SymbolMetadata) {
        self.symbols.insert(metadata.name.clone(), metadata);
    }

    pub fn get(&self, name: &str) -> Option<&SymbolMetadata> {
        self.symbols.get(name)
    }

    pub fn update_timestamp(&mut self, name: &str) {
        if let Some(symbol) = self.symbols.get_mut(name) {
            symbol.last_modified = Utc::now();
        }
    }

    pub fn list_by_type(&self, symbol_type: SymbolType) -> Vec<&SymbolMetadata> {
        self.symbols.values()
            .filter(|s| s.symbol_type == symbol_type)
            .collect()
    }

    /// Captures a snapshot for the debugger
    pub fn snapshot(&self) -> Vec<SymbolMetadata> {
        self.symbols.values().cloned().collect()
    }
}
