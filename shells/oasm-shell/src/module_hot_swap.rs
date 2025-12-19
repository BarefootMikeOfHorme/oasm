#![allow(dead_code)]
use std::collections::HashMap;
#[derive(Debug, Clone)] pub struct ActiveModule { pub name: String, pub version: String }
#[derive(Default)] pub struct HotSwap { map: HashMap<String, ActiveModule> }
impl HotSwap {
    pub fn new() -> Self { Self::default() }
    pub fn activate(&mut self, name: impl Into<String>, version: impl Into<String>) {
        let n = name.into();
        self.map.insert(n.clone(), ActiveModule { name: n, version: version.into() });
    }
    pub fn current(&self, name: &str) -> Option<&ActiveModule> { self.map.get(name) }
}
