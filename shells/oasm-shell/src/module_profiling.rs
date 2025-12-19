#![allow(dead_code)]
use std::time::{Duration, Instant};
pub struct Timer { start: Instant }
impl Timer {
    pub fn start() -> Self { Self { start: Instant::now() } }
    pub fn elapsed(&self) -> Duration { self.start.elapsed() }
    pub fn report(&self, label: &str) {
        println!("Timer [{label}] elapsed: {:?}", self.elapsed());
    }
}
