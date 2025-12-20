use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the overall 'compilable' state of the OASM environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilableState {
    pub is_compilable: bool,
    pub manifest_score: f32, // 0.0 to 1.0
    pub lint_score: f32,
    pub type_score: f32,
    pub test_score: f32,
    pub critical_errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl CompilableState {
    pub fn new() -> Self {
        Self {
            is_compilable: false,
            manifest_score: 0.0,
            lint_score: 0.0,
            type_score: 0.0,
            test_score: 0.0,
            critical_errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Determines if the overall system is in a healthy state for compilation
    pub fn update_status(&mut self) {
        let scores_ok = self.manifest_score > 0.9 
            && self.lint_score > 0.7 
            && self.type_score > 0.9 
            && self.test_score > 0.8;
        
        self.is_compilable = scores_ok && self.critical_errors.is_empty();
    }
}

/// Aggregates state from various OASM components
pub struct StateEvaluator {
    // In a real scenario, this might hold references to registries or databases
}

impl StateEvaluator {
    pub fn new() -> Self {
        Self {}
    }

    /// Evaluates the current project state
    /// In this implementation, it's a mock that would be populated by the daemon's diagnostics
    pub fn evaluate(&self, diagnostics: &HashMap<String, String>) -> CompilableState {
        let mut state = CompilableState::new();

        // Simple mock evaluation logic based on diagnostic keys
        if diagnostics.contains_key("manifest_ok") {
            state.manifest_score = 1.0;
        } else {
            state.critical_errors.push("Missing or invalid manifest".to_string());
        }

        if let Some(lint) = diagnostics.get("lint_warnings") {
            let count = lint.parse::<usize>().unwrap_or(0);
            state.lint_score = (1.0 - (count as f32 * 0.1)).max(0.0);
        } else {
            state.lint_score = 1.0;
        }

        state.type_score = 1.0; // Assume types are fine unless reported otherwise
        state.test_score = 1.0; // Assume tests are fine unless reported otherwise

        state.update_status();
        state
    }
}

impl Default for StateEvaluator {
    fn default() -> Self {
        Self::new()
    }
}
