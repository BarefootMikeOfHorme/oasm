#![allow(dead_code)]
use std::collections::HashMap;
#[derive(Default)] pub struct LabelTable{addrs:HashMap<String,usize>}
impl LabelTable{pub fn new()->Self{Self::default()} pub fn define(&mut self,name:impl Into<String>,addr:usize){self.addrs.insert(name.into(),addr);} pub fn addr(&self,name:&str)->Option<usize>{self.addrs.get(name).copied()}}
