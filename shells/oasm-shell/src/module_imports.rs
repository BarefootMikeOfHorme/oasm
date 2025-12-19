#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
#[derive(Default)] pub struct Imports { map: HashMap<String, HashSet<String>> }
impl Imports {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, module: impl Into<String>, import: impl Into<String>) {
        self.map.entry(module.into()).or_default().insert(import.into());
    }
    pub fn list(&self, module: &str) -> Vec<String> {
        self.map.get(module).map(|s| s.iter().cloned().collect()).unwrap_or_default()
    }
}
