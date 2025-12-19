#![allow(dead_code)]
//! Binary inspection tools
pub fn inspect(bin: &[u8]) -> (usize, u32) {
    let size = bin.len();
    let checksum = bin.iter().fold(0u32, |acc, &b| acc.wrapping_add(b as u32));
    println!("Binary size: {size} bytes, checksum: {checksum}");
    (size, checksum)
}
