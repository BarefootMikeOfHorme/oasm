#![allow(dead_code)]
pub fn extract_headers(src: &str) -> Vec<String> {
    src.lines().filter_map(|l| {
        let t = l.trim();
        if t.starts_with("//") || t.starts_with("#") {
            Some(t.trim_start_matches('/').trim_start_matches('#').trim().to_string())
        } else { None }
    }).collect()
}
