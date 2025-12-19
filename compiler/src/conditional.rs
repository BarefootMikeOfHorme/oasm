#![allow(dead_code)]
pub fn eval_flag(input: &str) -> bool {
    matches!(input.trim().to_ascii_lowercase().as_str(), "true"|"1"|"yes"|"on"|"enabled")
}
pub fn if_else<T>(cond: bool, a: T, b: T) -> T { if cond { a } else { b } }
