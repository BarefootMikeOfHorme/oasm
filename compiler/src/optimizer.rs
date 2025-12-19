#![allow(dead_code)]
pub fn remove_nops(instrs:&[&str])->Vec<String>{instrs.iter().filter(|s|s.trim().to_ascii_uppercase()!="NOP").map(|s|s.to_string()).collect()}
pub fn collapse_repeats(instrs:&[&str])->Vec<String>{let mut out=Vec::new(); for s in instrs{if out.last().map(|t:&String|t==s).unwrap_or(false){continue;} out.push(s.to_string());} out}
