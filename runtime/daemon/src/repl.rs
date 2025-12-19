#![allow(dead_code)]
use std::collections::HashMap;
pub type Handler = fn(&[&str]) -> String;
pub struct Repl { handlers: HashMap<String, Handler> }
impl Repl {
    pub fn new() -> Self { Self { handlers: HashMap::new() } }
    pub fn register(&mut self, name: impl Into<String>, h: Handler) { self.handlers.insert(name.into(), h); }
    pub fn run_once(&self, line: &str) -> String {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.is_empty() { return String::new(); }
        let cmd = parts[0]; let args = &parts[1..];
        match self.handlers.get(cmd) { Some(h) => h(args), None => format!("unknown command: {cmd}") }
    }
}
