#![allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)] pub enum Directive { Include(String), Define(String,String), Section(String) }
pub fn parse_directive(line: &str) -> Option<Directive> {
    let t = line.trim();
    if t.starts_with(".include ") { Some(Directive::Include(t[9..].trim().to_string())) }
    else if t.starts_with(".define ") {
        let rest = t[8..].trim(); let mut parts = rest.splitn(2,' ');
        Some(Directive::Define(parts.next()?.to_string(), parts.next().unwrap_or("").to_string()))
    } else if t.starts_with(".section ") { Some(Directive::Section(t[9..].trim().to_string())) }
    else { None }
}
