#![allow(dead_code)]
//! Performance profiler
use std::time::{Duration, Instant};

pub struct Profiler {
    start: Instant,
}

impl Profiler {
    pub fn new() -> Self {
        Self { start: Instant::now() }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn report(&self, label: &str) {
        println!("Profiler [{label}] elapsed: {:?}", self.elapsed());
    }
}
