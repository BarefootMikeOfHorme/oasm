#![allow(dead_code)]
//! Structured constants (arrays, strings)
pub fn const_array(values: &[i32]) -> Vec<i32> {
    values.to_vec()
}

pub fn const_string(s: &str) -> String {
    s.to_string()
}
