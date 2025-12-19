#![allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Finding { pub rule: String, pub message: String, pub offset: usize }
pub fn lint_source(src: &str) -> Vec<Finding> {
    let mut out = Vec::new();
    if src.contains("goto") {
        out.push(Finding { rule: "no-goto".into(), message: "Avoid 'goto'".into(), offset: src.find("goto").unwrap_or(0) });
    }
    if src.trim().is_empty() {
        out.push(Finding { rule: "empty-source".into(), message: "Source is empty".into(), offset: 0 });
    }
    out
}
