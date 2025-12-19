#![allow(dead_code)]
use std::collections::HashMap;
pub type CommandText = String;
#[derive(Default)] pub struct CommandBlocks { blocks: HashMap<String, CommandText> }
impl CommandBlocks {
    pub fn new() -> Self { Self::default() }
    pub fn register(&mut self, name: impl Into<String>, body: impl Into<String>) { self.blocks.insert(name.into(), body.into()); }
    pub fn get(&self, name: &str) -> Option<&str> { self.blocks.get(name).map(|s| s.as_str()) }
}
