#![allow(dead_code)]
//! Plugin API for extensions
pub trait OasmPlugin {
    fn name(&self) -> &str;
    fn init(&mut self);
    fn execute(&self, input: &str) -> String;
    fn shutdown(&mut self);
}

pub struct PluginManager {
    plugins: Vec<Box<dyn OasmPlugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn register(&mut self, plugin: Box<dyn OasmPlugin>) {
        self.plugins.push(plugin);
    }

    pub fn run_all(&self, input: &str) -> Vec<String> {
        self.plugins.iter().map(|p| p.execute(input)).collect()
    }
}
