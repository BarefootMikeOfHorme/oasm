/// OASM Native Executor
/// Executes OASM instructions with command block batching support

use crate::context::{ContextManager, ExecutionContext, ContextError};
use crate::parser::{Instruction, Operand};
use crate::types::{Value, NativeTypeChecker, TypeChecker};

/// Execution result
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub outcome: ExecutionOutcome,
    pub output: Option<Value>,
    pub modified_objects: Vec<String>,
    pub duration_ms: u64,
}

/// Execution outcome
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionOutcome {
    Success,
    Failed { reason: String },
    PartialSuccess { completed: usize, total: usize },
}

/// Executor trait
pub trait InstructionExecutor {
    fn execute(&mut self, instruction: &Instruction, ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError>;
    fn execute_batch(&mut self, instructions: &[Instruction], ctx: &mut ExecutionContext) -> Result<BatchResult, ExecutorError>;
}

/// Batch execution result
#[derive(Debug, Clone)]
pub struct BatchResult {
    pub outcome: ExecutionOutcome,
    pub individual_results: Vec<ExecutionResult>,
    pub total_duration_ms: u64,
}

/// Executor errors
#[derive(Debug, Clone)]
pub enum ExecutorError {
    ContextError(String),
    InvalidInstruction { instruction: String, reason: String },
    TypeError { variable: String, error: String },
    RuntimeError(String),
}

impl From<ContextError> for ExecutorError {
    fn from(e: ContextError) -> Self {
        ExecutorError::ContextError(format!("{:?}", e))
    }
}

use std::collections::HashMap;
use std::sync::Arc;

/// Instruction handler trait
pub trait InstructionHandler: Send + Sync {
    fn execute(&self, operands: &[Operand], ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError>;
}

/// Instruction registry
pub struct InstructionRegistry {
    handlers: HashMap<String, Arc<dyn InstructionHandler>>,
}

impl InstructionRegistry {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register(&mut self, mnemonic: &str, handler: Arc<dyn InstructionHandler>) {
        self.handlers.insert(mnemonic.to_uppercase(), handler);
    }

    pub fn get(&self, mnemonic: &str) -> Option<Arc<dyn InstructionHandler>> {
        self.handlers.get(&mnemonic.to_uppercase()).cloned()
    }
}

impl Default for InstructionRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        registry.register("CREATE", Arc::new(CreateHandler));
        registry.register("SET", Arc::new(SetHandler));
        registry.register("EXTRUDE", Arc::new(ExtrudeHandler));
        registry.register("FILLET", Arc::new(FilletHandler));
        registry.register("MOVE", Arc::new(MoveHandler));
        registry.register("ROTATE", Arc::new(RotateHandler));
        registry.register("SCALE", Arc::new(ScaleHandler));
        registry.register("BOOLEAN", Arc::new(BooleanHandler));
        registry.register("VALIDATE", Arc::new(ValidateHandler));
        registry.register("EXPORT", Arc::new(ExportHandler));
        registry
    }
}

struct CreateHandler;
impl InstructionHandler for CreateHandler {
    fn execute(&self, operands: &[Operand], ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError> {
        let start = std::time::Instant::now();

        if operands.is_empty() {
            return Err(ExecutorError::InvalidInstruction {
                instruction: "CREATE".to_string(),
                reason: "Missing object type".to_string(),
            });
        }

        let object_type = match &operands[0] {
            Operand::Identifier(name) => name.clone(),
            _ => return Err(ExecutorError::InvalidInstruction {
                instruction: "CREATE".to_string(),
                reason: "Expected identifier".to_string(),
            }),
        };

        let object_id = ctx.create_object(object_type, None)?;
        ctx.next_seq();

        Ok(ExecutionResult {
            outcome: ExecutionOutcome::Success,
            output: Some(Value::String(object_id.clone())),
            modified_objects: vec![object_id],
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
}

struct SetHandler;
impl InstructionHandler for SetHandler {
    fn execute(&self, operands: &[Operand], ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError> {
        let start = std::time::Instant::now();
        let type_checker = NativeTypeChecker;

        if operands.is_empty() {
            return Err(ExecutorError::InvalidInstruction {
                instruction: "SET".to_string(),
                reason: "Missing assignment".to_string(),
            });
        }

        match &operands[0] {
            Operand::Assignment { target, value } => {
                let val = match &**value {
                    Operand::Literal(v) => v.clone(),
                    _ => return Err(ExecutorError::RuntimeError("Cannot extract value".to_string())),
                };

                if let Ok(var) = ctx.get_variable(target) {
                    let inferred_type = type_checker.infer_type(&val);
                    if let Err(type_err) = type_checker.check_assignment(&var.var_type, &inferred_type) {
                        return Err(ExecutorError::TypeError {
                            variable: target.clone(),
                            error: format!("{}", type_err),
                        });
                    }
                }

                ctx.assign_variable(target, val)?;
                ctx.next_seq();

                Ok(ExecutionResult {
                    outcome: ExecutionOutcome::Success,
                    output: None,
                    modified_objects: vec![],
                    duration_ms: start.elapsed().as_millis() as u64,
                })
            }
            _ => Err(ExecutorError::InvalidInstruction {
                instruction: "SET".to_string(),
                reason: "Expected assignment".to_string(),
            }),
        }
    }
}

struct ExtrudeHandler;
impl InstructionHandler for ExtrudeHandler {
    fn execute(&self, operands: &[Operand], _ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError> {
        let start = std::time::Instant::now();
        // Extrude logic: EXTRUDE object, distance
        if operands.len() < 2 {
            return Err(ExecutorError::InvalidInstruction {
                instruction: "EXTRUDE".to_string(),
                reason: "Missing operands (expected: object, distance)".to_string(),
            });
        }
        
        // Implementation details omitted for brevity, but this would update mesh data
        Ok(ExecutionResult {
            outcome: ExecutionOutcome::Success,
            output: None,
            modified_objects: vec![], // would be [object_id]
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
}

struct FilletHandler;
impl InstructionHandler for FilletHandler {
    fn execute(&self, _operands: &[Operand], _ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError> {
        Ok(ExecutionResult {
            outcome: ExecutionOutcome::Success,
            output: None,
            modified_objects: vec![],
            duration_ms: 0,
        })
    }
}

struct MoveHandler;
impl InstructionHandler for MoveHandler {
    fn execute(&self, _operands: &[Operand], _ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError> {
        Ok(ExecutionResult {
            outcome: ExecutionOutcome::Success,
            output: None,
            modified_objects: vec![],
            duration_ms: 0,
        })
    }
}

struct RotateHandler;
impl InstructionHandler for RotateHandler {
    fn execute(&self, _operands: &[Operand], _ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError> {
        Ok(ExecutionResult {
            outcome: ExecutionOutcome::Success,
            output: None,
            modified_objects: vec![],
            duration_ms: 0,
        })
    }
}

struct ScaleHandler;
impl InstructionHandler for ScaleHandler {
    fn execute(&self, _operands: &[Operand], _ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError> {
        Ok(ExecutionResult {
            outcome: ExecutionOutcome::Success,
            output: None,
            modified_objects: vec![],
            duration_ms: 0,
        })
    }
}

struct BooleanHandler;
impl InstructionHandler for BooleanHandler {
    fn execute(&self, _operands: &[Operand], _ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError> {
        Ok(ExecutionResult {
            outcome: ExecutionOutcome::Success,
            output: None,
            modified_objects: vec![],
            duration_ms: 0,
        })
    }
}

struct ValidateHandler;
impl InstructionHandler for ValidateHandler {
    fn execute(&self, _operands: &[Operand], _ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError> {
        Ok(ExecutionResult {
            outcome: ExecutionOutcome::Success,
            output: None,
            modified_objects: vec![],
            duration_ms: 0,
        })
    }
}

struct ExportHandler;
impl InstructionHandler for ExportHandler {
    fn execute(&self, _operands: &[Operand], _ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError> {
        Ok(ExecutionResult {
            outcome: ExecutionOutcome::Success,
            output: None,
            modified_objects: vec![],
            duration_ms: 0,
        })
    }
}

/// Native executor
pub struct NativeExecutor {
    registry: InstructionRegistry,
}

impl NativeExecutor {
    pub fn new() -> Self {
        Self { registry: InstructionRegistry::default() }
    }

    pub fn with_registry(registry: InstructionRegistry) -> Self {
        Self { registry }
    }
}

impl InstructionExecutor for NativeExecutor {
    fn execute(&mut self, instruction: &Instruction, ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError> {
        if let Some(handler) = self.registry.get(&instruction.mnemonic) {
            handler.execute(&instruction.operands, ctx)
        } else {
            // Default behavior for unknown instructions (fallback to success for now, as in original)
            Ok(ExecutionResult {
                outcome: ExecutionOutcome::Success,
                output: None,
                modified_objects: vec![],
                duration_ms: 0,
            })
        }
    }

    fn execute_batch(&mut self, instructions: &[Instruction], ctx: &mut ExecutionContext) -> Result<BatchResult, ExecutorError> {
        let start = std::time::Instant::now();
        let mut individual_results = Vec::new();
        let mut completed = 0;

        for instruction in instructions {
            match self.execute(instruction, ctx) {
                Ok(result) => {
                    if result.outcome == ExecutionOutcome::Success {
                        completed += 1;
                    }
                    individual_results.push(result);
                }
                Err(_) => break,
            }
        }

        let outcome = if completed == instructions.len() {
            ExecutionOutcome::Success
        } else {
            ExecutionOutcome::PartialSuccess { completed, total: instructions.len() }
        };

        Ok(BatchResult {
            outcome,
            individual_results,
            total_duration_ms: start.elapsed().as_millis() as u64,
        })
    }
}

impl Default for NativeExecutor {
    fn default() -> Self {
        Self::new()
    }
}
