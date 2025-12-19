#![allow(dead_code)]
//! Symbolic debugging support
#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub addr: usize,
}

pub fn debug_symbols(symbols: &[Symbol]) {
    for sym in symbols {
        println!("Symbol {} at {:#X}", sym.name, sym.addr);
    }
}
