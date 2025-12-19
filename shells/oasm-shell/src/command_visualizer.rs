#![allow(dead_code)]
pub fn visualize(cmd: &str) -> String {
    let mut out = String::new();
    for tok in cmd.split_whitespace() {
        if tok.starts_with('-') { out.push_str(&format!("[FLAG:{}] ", tok)); }
        else { out.push_str(&format!("[CMD:{}] ", tok)); }
    }
    out.trim_end().to_string()
}
