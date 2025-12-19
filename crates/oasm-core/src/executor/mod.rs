/// OASM Native Executor
/// Executes OASM instructions with command block batching support

use crate::context::{ContextManager, ExecutionContext, ContextError};
use crate::parser::{Instruction, Operand};
use crate::types::{Value, NativeTypeChecker};

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
    RuntimeError(String),
}

impl From<ContextError> for ExecutorError {
    fn from(e: ContextError) -> Self {
        ExecutorError::ContextError(format!("{:?}", e))
    }
}

/// Native executor
pub struct NativeExecutor {
    type_checker: NativeTypeChecker,
}

impl NativeExecutor {
    pub fn new() -> Self {
        Self { type_checker: NativeTypeChecker }
    }

    fn execute_create(&mut self, operands: &[Operand], ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError> {
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

    fn execute_set(&mut self, operands: &[Operand], ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError> {
        let start = std::time::Instant::now();

        if operands.is_empty() {
            return Err(ExecutorError::InvalidInstruction {
                instruction: "SET".to_string(),
                reason: "Missing assignment".to_string(),
            });
        }

        match &operands[0] {
            Operand::Assignment { target, value } => {
                let val = self.extract_value(value)?;
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

    fn extract_value(&self, operand: &Operand) -> Result<Value, ExecutorError> {
        match operand {
            Operand::Literal(val) => Ok(val.clone()),
            _ => Err(ExecutorError::RuntimeError("Cannot extract value".to_string())),
        }
    }
}

impl InstructionExecutor for NativeExecutor {
    fn execute(&mut self, instruction: &Instruction, ctx: &mut ExecutionContext) -> Result<ExecutionResult, ExecutorError> {
        match instruction.mnemonic.as_str() {
            "CREATE" => self.execute_create(&instruction.operands, ctx),
            "SET" => self.execute_set(&instruction.operands, ctx),
            _ => Ok(ExecutionResult {
                outcome: ExecutionOutcome::Success,
                output: None,
                modified_objects: vec![],
                duration_ms: 0,
            }),
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
