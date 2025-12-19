#![allow(dead_code)]
//! Relocation viewer utilities
pub fn show_relocations(relocs: &[usize]) {
    for (i, r) in relocs.iter().enumerate() {
        println!("Relocation {} at address {}", i, r);
    }
}
