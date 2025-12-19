#![allow(dead_code)]
//! Build reproducibility logs
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn log_build(id: &str) {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut f = OpenOptions::new().create(true).append(true).open("builds.log").unwrap();
    writeln!(f, "{ts} | build={id}").unwrap();
    println!("Logged build {id} at {ts}");
}
