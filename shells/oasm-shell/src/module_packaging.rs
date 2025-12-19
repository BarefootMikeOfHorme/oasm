#![allow(dead_code)]
pub fn package(name: &str, files: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(name.as_bytes());
    out.push(0);
    for (fname, data) in files {
        out.extend_from_slice(fname.as_bytes());
        out.push(0);
        out.extend_from_slice(&data.len().to_le_bytes());
        out.extend_from_slice(data);
    }
    out
}
