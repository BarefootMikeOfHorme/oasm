#![allow(dead_code)]
//! Cross-target toolchain manager
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Toolchain {
    pub name: String,
    pub compiler: String,
    pub linker: String,
    pub runner: String,
}

pub struct ToolchainManager {
    toolchains: HashMap<String, Toolchain>,
}

impl ToolchainManager {
    pub fn new() -> Self {
        Self { toolchains: HashMap::new() }
    }

    pub fn register(&mut self, tc: Toolchain) {
        self.toolchains.insert(tc.name.clone(), tc);
    }

    pub fn get(&self, name: &str) -> Option<&Toolchain> {
        self.toolchains.get(name)
    }

    pub fn list(&self) -> Vec<&Toolchain> {
        self.toolchains.values().collect()
    }
}
