#![allow(dead_code)]
//! Relocation viewer hooks
pub fn view(relocs: &[usize]) -> String {
    let mut out = String::from("Relocations:\n");
    for (i, r) in relocs.iter().enumerate() {
        out.push_str(&format!("- [{}] addr=0x{r:X}\n", i));
    }
    println!("{}", out);
    out
}
