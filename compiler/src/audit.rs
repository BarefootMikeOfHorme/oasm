#![allow(dead_code)]
//! Audit log generator for builds
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_event(event: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("build_audit.log")
        .unwrap();
    writeln!(file, "{}", event).unwrap();
}
