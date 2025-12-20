/// OASM Command Block Builder
/// Batches instructions together for atomic execution with testing/repair loops

use crate::parser::Instruction;
use crate::context::{RunId, Seq};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Block types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockType {
    RepairBlock,      // Fix code issues
    LintBlock,        // Check code quality
    TestBlock,        // Run tests
    CADBlock,         // CAD operations
    ValidationBlock,  // Validation checks
    CustomBlock { name: String },
}

/// Execution mode
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutionMode {
    Sequential,       // One after another
    Parallel,         // All at once (if safe)
    ConditionalParallel, // Parallel where possible
}

/// Command block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandBlock {
    pub block_id: String,
    pub block_type: BlockType,
    pub instructions: Vec<Instruction>,
    pub targets: Vec<String>,
    pub rules: Vec<String>,
    pub execution_mode: ExecutionMode,
    pub checkpoint_before: bool,
    pub checkpoint_after: bool,
    pub test_after_execution: bool,
    pub repair_on_failure: bool,
    pub created: DateTime<Utc>,
    pub run_id: RunId,
    pub seq: Seq,
    pub require_compilable_state: bool, // New flag for smart state awareness
}

/// Command block builder trait
pub trait CommandBlockBuilder {
    fn new(block_type: BlockType) -> Self;
    fn add_instruction(&mut self, instruction: Instruction) -> &mut Self;
    fn add_target(&mut self, target: String) -> &mut Self;
    fn add_rule(&mut self, rule_id: String) -> &mut Self;
    fn set_execution_mode(&mut self, mode: ExecutionMode) -> &mut Self;
    fn enable_checkpoints(&mut self) -> &mut Self;
    fn enable_testing(&mut self) -> &mut Self;
    fn enable_repair_loop(&mut self) -> &mut Self;
    fn require_compilable_state(&mut self) -> &mut Self; // New method
    fn build(self) -> Result<CommandBlock, BuildError>;
}

/// Builder errors
#[derive(Debug, Clone)]
pub enum BuildError {
    NoInstructions,
    InvalidBlockType,
    ConflictingOptions,
}

/// Batch builder implementation
pub struct BatchBuilder {
    block_type: BlockType,
    instructions: Vec<Instruction>,
    targets: Vec<String>,
    rules: Vec<String>,
    execution_mode: ExecutionMode,
    checkpoint_before: bool,
    checkpoint_after: bool,
    test_after_execution: bool,
    repair_on_failure: bool,
    require_compilable_state: bool,
    run_id: RunId,
    seq: Seq,
}

impl CommandBlockBuilder for BatchBuilder {
    fn new(block_type: BlockType) -> Self {
        Self {
            block_type,
            instructions: Vec::new(),
            targets: Vec::new(),
            rules: Vec::new(),
            execution_mode: ExecutionMode::Sequential,
            checkpoint_before: false,
            checkpoint_after: false,
            test_after_execution: false,
            repair_on_failure: false,
            require_compilable_state: false,
            run_id: RunId::new(),
            seq: Seq::zero(),
        }
    }

    fn add_instruction(&mut self, instruction: Instruction) -> &mut Self {
        self.instructions.push(instruction);
        self
    }

    fn add_target(&mut self, target: String) -> &mut Self {
        self.targets.push(target);
        self
    }

    fn add_rule(&mut self, rule_id: String) -> &mut Self {
        self.rules.push(rule_id);
        self
    }

    fn set_execution_mode(&mut self, mode: ExecutionMode) -> &mut Self {
        self.execution_mode = mode;
        self
    }

    fn enable_checkpoints(&mut self) -> &mut Self {
        self.checkpoint_before = true;
        self.checkpoint_after = true;
        self
    }

    fn enable_testing(&mut self) -> &mut Self {
        self.test_after_execution = true;
        self
    }

    fn enable_repair_loop(&mut self) -> &mut Self {
        self.repair_on_failure = true;
        self
    }

    fn require_compilable_state(&mut self) -> &mut Self {
        self.require_compilable_state = true;
        self
    }

    fn build(self) -> Result<CommandBlock, BuildError> {
        if self.instructions.is_empty() {
            return Err(BuildError::NoInstructions);
        }

        let block_id = format!("block_{}_{}", self.run_id, self.seq.0);

        Ok(CommandBlock {
            block_id,
            block_type: self.block_type,
            instructions: self.instructions,
            targets: self.targets,
            rules: self.rules,
            execution_mode: self.execution_mode,
            checkpoint_before: self.checkpoint_before,
            checkpoint_after: self.checkpoint_after,
            test_after_execution: self.test_after_execution,
            repair_on_failure: self.repair_on_failure,
            require_compilable_state: self.require_compilable_state,
            created: Utc::now(),
            run_id: self.run_id,
            seq: self.seq,
        })
    }
}

/// Testing and repair loop configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestingConfig {
    pub run_tests: bool,
    pub test_types: Vec<TestType>,
    pub failure_threshold: f64,  // 0.0 - 1.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestType {
    UnitTests,
    IntegrationTests,
    ValidationChecks,
    TopologyChecks,  // CAD-specific
}

/// Repair loop configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairConfig {
    pub enabled: bool,
    pub max_attempts: usize,
    pub repair_strategies: Vec<RepairStrategy>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RepairStrategy {
    RetryWithBackoff,
    ApplyAlternativeMethod,
    RollbackAndSkip,
    RequestUserInput,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Operand;

    #[test]
    fn test_build_simple_block() {
        let mut builder = BatchBuilder::new(BlockType::RepairBlock);

        builder
            .add_instruction(Instruction {
                mnemonic: "CREATE".to_string(),
                operands: vec![Operand::Identifier("gear".to_string())],
                line_number: 1,
            })
            .add_target("src/main.rs".to_string())
            .add_rule("fix_unsafe".to_string());

        let block = builder.build().unwrap();

        assert_eq!(block.block_type, BlockType::RepairBlock);
        assert_eq!(block.instructions.len(), 1);
        assert_eq!(block.targets.len(), 1);
        assert_eq!(block.rules.len(), 1);
    }

    #[test]
    fn test_build_with_testing_and_repair() {
        let mut builder = BatchBuilder::new(BlockType::TestBlock);

        builder
            .add_instruction(Instruction {
                mnemonic: "VALIDATE".to_string(),
                operands: vec![],
                line_number: 1,
            })
            .enable_testing()
            .enable_repair_loop()
            .enable_checkpoints();

        let block = builder.build().unwrap();

        assert!(block.test_after_execution);
        assert!(block.repair_on_failure);
        assert!(block.checkpoint_before);
        assert!(block.checkpoint_after);
    }

    #[test]
    fn test_empty_block_fails() {
        let builder = BatchBuilder::new(BlockType::RepairBlock);
        assert!(builder.build().is_err());
    }
}
