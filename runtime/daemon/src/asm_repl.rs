#![allow(dead_code)]
//! Assembler REPL integration
use std::io::{self, Write};

pub fn start_repl() {
    println!("OASM REPL (type 'exit' to quit)");
    loop {
        print!("oasm> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() { break; }
        let s = line.trim();
        if s.eq_ignore_ascii_case("exit") { break; }
        println!("echo: {s}");
    }
}
