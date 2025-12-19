#![allow(dead_code)]
#[derive(Debug)]
pub struct BinSummary { pub size: usize, pub checksum: u32, pub preview_hex: String }
pub fn summarize(bytes: &[u8]) -> BinSummary {
    let size = bytes.len();
    let checksum = bytes.iter().fold(0u32, |acc, &b| acc.wrapping_add(b as u32));
    let preview_hex = bytes.iter().take(32).map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ");
    BinSummary { size, checksum, preview_hex }
}
