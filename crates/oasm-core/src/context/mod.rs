/// OASM Execution Context Manager
/// Manages execution state: variables, objects, scopes, run tracking

use crate::types::{OasmType, Value};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;
use crate::symbol_table::{SymbolTable, SymbolMetadata, SymbolType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RunId(pub Uuid);

impl RunId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
}

impl Default for RunId {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Seq(pub u64);

impl Seq {
    pub fn zero() -> Self { Self(0) }
    pub fn next(&self) -> Self { Self(self.0 + 1) }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Actor {
    Human { username: String },
    Automation { rule_id: String },
    AI { model: String, confidence: f64 },
    System,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub var_type: OasmType,
    pub value: Option<Value>,
    pub mutable: bool,
}

#[derive(Debug, Clone)]
pub struct Object {
    pub id: String,
    pub object_type: String,
    pub properties: HashMap<String, Value>,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub name: String,
    pub variables: HashMap<String, Variable>,
}

impl Scope {
    pub fn new(name: String) -> Self {
        Self { name, variables: HashMap::new() }
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub run_id: RunId,
    pub seq: Seq,
    pub actor: Actor,
    pub working_directory: PathBuf,
    pub scope_stack: Vec<Scope>,
    pub objects: HashMap<String, Object>,
    pub symbol_table: SymbolTable, // New: tracking all symbols for debugging
    pub created: DateTime<Utc>,
}

impl ExecutionContext {
    pub fn new(actor: Actor, working_directory: PathBuf) -> Self {
        Self {
            run_id: RunId::new(),
            seq: Seq::zero(),
            actor,
            working_directory,
            scope_stack: vec![Scope::new("global".to_string())],
            objects: HashMap::new(),
            symbol_table: SymbolTable::new(),
            created: Utc::now(),
        }
    }
}

pub trait ContextManager {
    fn push_scope(&mut self, name: String);
    fn pop_scope(&mut self) -> Result<Scope, ContextError>;
    fn declare_variable(&mut self, name: String, var_type: OasmType, mutable: bool) -> Result<(), ContextError>;
    fn assign_variable(&mut self, name: &str, value: Value) -> Result<(), ContextError>;
    fn get_variable(&self, name: &str) -> Result<&Variable, ContextError>;
    fn create_object(&mut self, object_type: String, id: Option<String>) -> Result<String, ContextError>;
    fn get_object(&self, id: &str) -> Result<&Object, ContextError>;
}

#[derive(Debug, Clone)]
pub enum ContextError {
    ScopeStackEmpty,
    VariableAlreadyDefined(String),
    VariableNotFound(String),
    ObjectNotFound(String),
}

impl ExecutionContext {
    pub fn next_seq(&mut self) {
        self.seq = self.seq.next();
    }
}

impl ContextManager for ExecutionContext {
    fn push_scope(&mut self, name: String) {
        self.scope_stack.push(Scope::new(name));
    }

    fn pop_scope(&mut self) -> Result<Scope, ContextError> {
        if self.scope_stack.len() <= 1 {
            Err(ContextError::ScopeStackEmpty)
        } else {
            Ok(self.scope_stack.pop().unwrap())
        }
    }

    fn declare_variable(&mut self, name: String, var_type: OasmType, mutable: bool) -> Result<(), ContextError> {
        let scope = self.scope_stack.last_mut().unwrap();
        if scope.variables.contains_key(&name) {
            return Err(ContextError::VariableAlreadyDefined(name));
        }
        scope.variables.insert(name.clone(), Variable {
            name: name.clone(),
            var_type: var_type.clone(),
            value: None,
            mutable,
        });

        // Track in symbol table
        self.symbol_table.insert(SymbolMetadata {
            name,
            symbol_type: SymbolType::Variable,
            data_type: var_type,
            created_at: Utc::now(),
            last_modified: Utc::now(),
            source_line: 0, // In real usage, pass from instruction
        });
        Ok(())
    }

    fn assign_variable(&mut self, name: &str, value: Value) -> Result<(), ContextError> {
        for scope in self.scope_stack.iter_mut().rev() {
            if let Some(var) = scope.variables.get_mut(name) {
                var.value = Some(value);
                self.symbol_table.update_timestamp(name);
                return Ok(());
            }
        }
        Err(ContextError::VariableNotFound(name.to_string()))
    }

    fn get_variable(&self, name: &str) -> Result<&Variable, ContextError> {
        for scope in self.scope_stack.iter().rev() {
            if let Some(var) = scope.variables.get(name) {
                return Ok(var);
            }
        }
        Err(ContextError::VariableNotFound(name.to_string()))
    }

    fn create_object(&mut self, object_type: String, id: Option<String>) -> Result<String, ContextError> {
        let object_id = id.unwrap_or_else(|| format!("{}_{:04}", object_type, self.seq.0));
        let object = Object {
            id: object_id.clone(),
            object_type: object_type.clone(),
            properties: HashMap::new(),
            created: Utc::now(),
        };
        self.objects.insert(object_id.clone(), object);

        // Track in symbol table
        self.symbol_table.insert(SymbolMetadata {
            name: object_id.clone(),
            symbol_type: SymbolType::Object,
            data_type: OasmType::Object { object_type },
            created_at: Utc::now(),
            last_modified: Utc::now(),
            source_line: 0,
        });

        Ok(object_id)
    }

    fn get_object(&self, id: &str) -> Result<&Object, ContextError> {
        self.objects.get(id).ok_or_else(|| ContextError::ObjectNotFound(id.to_string()))
    }
}

impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ContextError::ScopeStackEmpty => write!(f, "Scope stack is empty"),
            ContextError::VariableAlreadyDefined(name) => write!(f, "Variable '{}' already defined", name),
            ContextError::VariableNotFound(name) => write!(f, "Variable '{}' not found", name),
            ContextError::ObjectNotFound(id) => write!(f, "Object '{}' not found", id),
        }
    }
}

impl std::error::Error for ContextError {}

impl std::fmt::Display for RunId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
