#![allow(dead_code)]
use std::collections::HashMap;
pub fn expand(template:&str,vars:&HashMap<String,String>)->String{
    let mut out=String::new(); let mut i=0; let bytes=template.as_bytes();
    while i<bytes.len(){ if bytes[i]==b'$'&&i+1<bytes.len()&&bytes[i+1]==b'{'{
        let start=i+2; if let Some(end_rel)=template[start..].find('}'){let end=start+end_rel; let key=&template[start..end]; out.push_str(vars.get(key).map(|s|s.as_str()).unwrap_or("")); i=end+1; continue;}}
        out.push(bytes[i] as char); i+=1;} out }
