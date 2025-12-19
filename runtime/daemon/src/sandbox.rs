#![allow(dead_code)]
//! Sandbox execution environment
pub struct Sandbox {
    pub memory_limit: usize,
    pub time_limit_ms: u64,
}

impl Sandbox {
    pub fn new(memory_limit: usize, time_limit_ms: u64) -> Self {
        Self { memory_limit, time_limit_ms }
    }

    pub fn run<F>(&self, task: F)
    where
        F: FnOnce(),
    {
        println!("Running in sandbox (mem={} bytes, time={} ms)", self.memory_limit, self.time_limit_ms);
        task();
    }
}
