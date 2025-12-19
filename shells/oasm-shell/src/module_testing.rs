#![allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TestCase { pub name: String, pub run: fn() -> bool }
pub fn run_all(cases: &[TestCase]) -> (usize, usize) {
    let mut pass = 0;
    for c in cases { if (c.run)() { pass += 1; } }
    (pass, cases.len() - pass)
}
