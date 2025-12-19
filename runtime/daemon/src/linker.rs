#![allow(dead_code)]
//! Linker integration routines
use std::collections::HashMap;

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub address: usize,
}

pub struct Linker {
    pub symbols: HashMap<String, Symbol>,
}

impl Linker {
    pub fn new() -> Self {
        Self { symbols: HashMap::new() }
    }

    pub fn add_symbol(&mut self, name: &str, address: usize) {
        self.symbols.insert(name.to_string(), Symbol { name: name.to_string(), address });
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub fn link_objects(&self, objects: Vec<Vec<u8>>) -> Vec<u8> {
        objects.into_iter().flatten().collect()
    }
}
