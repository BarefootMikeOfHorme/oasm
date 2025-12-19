#![allow(dead_code)]
//! Cross-reference utilities
use std::collections::HashMap;

pub fn build_xref(symbols: &[String]) -> HashMap<String, usize> {
    symbols.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect()
}
