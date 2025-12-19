#![allow(dead_code)]
#[derive(Debug, Clone)] pub struct Validation { pub ok: bool, pub message: String }
pub fn validate_nonempty(s: &str) -> Validation {
    if s.trim().is_empty() { Validation { ok: false, message: "command is empty".into() } }
    else { Validation { ok: true, message: "ok".into() } }
}
