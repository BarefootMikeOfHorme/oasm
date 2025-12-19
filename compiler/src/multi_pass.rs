#![allow(dead_code)]
//! Multi-pass resolution engine
pub struct MultiPass {
    pub passes: usize,
}

impl MultiPass {
    pub fn new() -> Self {
        Self { passes: 0 }
    }

    pub fn run_pass(&mut self) {
        self.passes += 1;
        println!("Running pass {}", self.passes);
    }
}
