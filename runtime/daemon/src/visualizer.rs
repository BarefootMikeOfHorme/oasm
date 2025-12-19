#![allow(dead_code)]
pub fn visualize(nodes: &[&str], edges: &[(usize, usize)]) -> String {
    let mut out = String::new();
    out.push_str("Nodes:\n");
    for (i, n) in nodes.iter().enumerate() { out.push_str(&format!("  [{}] {}\n", i, n)); }
    out.push_str("Edges:\n");
    for (a, b) in edges { out.push_str(&format!("  {} -> {}\n", a, b)); }
    out
}
