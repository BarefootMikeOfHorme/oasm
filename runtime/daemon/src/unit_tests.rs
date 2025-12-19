#![allow(dead_code)]
//! Unit test harness (minimal but real)
pub fn run_tests() -> bool {
    fn add(a: i32, b: i32) -> i32 { a + b }
    let ok = add(2, 2) == 4 && add(-1, 1) == 0;
    println!("Tests: {}", if ok { "passed" } else { "failed" });
    ok
}
