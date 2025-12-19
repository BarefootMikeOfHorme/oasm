#![allow(dead_code)]
use std::collections::HashMap;
#[derive(Debug, Clone)] pub enum ConstValue { Int(i64), Str(String), Bytes(Vec<u8>) }
#[derive(Default)] pub struct ConstRegistry { map: HashMap<String, ConstValue> }
impl ConstRegistry {
    pub fn new() -> Self { Self::default() }
    pub fn set(&mut self, name: impl Into<String>, val: ConstValue) { self.map.insert(name.into(), val); }
    pub fn get(&self, name: &str) -> Option<&ConstValue> { self.map.get(name) }
}
