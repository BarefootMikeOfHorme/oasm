#![allow(dead_code)]
#[derive(Debug,Clone)] pub struct Reloc{pub offset:usize,pub value:usize}
pub fn apply_relocs(buf:&mut[u8],relocs:&[Reloc]){for r in relocs{let bytes=(r.value as u32).to_le_bytes(); if r.offset+4<=buf.len(){buf[r.offset..r.offset+4].copy_from_slice(&bytes);}}}
