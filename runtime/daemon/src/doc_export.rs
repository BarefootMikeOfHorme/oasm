#![allow(dead_code)]
//! Inline documentation exporter
use std::fs::File;
use std::io::Write;

pub fn export_docs(path: &str, title: &str, body: &str) {
    let mut f = File::create(path).unwrap();
    writeln!(f, "# {title}\n\n{body}").unwrap();
    println!("Docs exported to {path}");
}
