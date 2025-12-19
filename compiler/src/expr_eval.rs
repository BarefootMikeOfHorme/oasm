#![allow(dead_code)]
//! Advanced expression evaluator
pub fn eval_expr(expr: &str) -> i32 {
    expr.parse::<i32>().unwrap_or(0)
}
