#![allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StageResult { pub name: String, pub ok: bool, pub message: String }
pub fn run_pipeline<F>(stages: &[&str], mut runner: F) -> Vec<StageResult>
where F: FnMut(&str) -> Result<(), String> {
    let mut results = Vec::new();
    for s in stages {
        match runner(s) {
            Ok(()) => results.push(StageResult { name: s.to_string(), ok: true, message: "ok".into() }),
            Err(e) => results.push(StageResult { name: s.to_string(), ok: false, message: e }),
        }
    }
    results
}
