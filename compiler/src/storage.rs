#![allow(dead_code)]
use std::collections::HashMap;
#[derive(Default)] pub struct Segments{data:HashMap<String,Vec<u8>>}
impl Segments{pub fn new()->Self{Self::default()} pub fn put(&mut self,name:impl Into<String>,bytes:Vec<u8>){self.data.insert(name.into(),bytes);} pub fn get(&self,name:&str)->Option<&[u8]>{self.data.get(name).map(|v|v.as_slice())}}
