#![allow(dead_code)]
//! Cross-assembly for different architectures
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TargetArch {
    X86_64,
    ARM64,
    RISCV,
}

pub struct CrossAssembler {
    backends: HashMap<TargetArch, Box<dyn Fn(&[u8]) -> Vec<u8>>>,
}

impl CrossAssembler {
    pub fn new() -> Self {
        let mut backends = HashMap::new();
        backends.insert(TargetArch::X86_64, Box::new(|bytes| bytes.to_vec()));
        backends.insert(TargetArch::ARM64, Box::new(|bytes| bytes.to_vec()));
        backends.insert(TargetArch::RISCV, Box::new(|bytes| bytes.to_vec()));
        Self { backends }
    }

    pub fn assemble(&self, arch: TargetArch, ir: &[u8]) -> Vec<u8> {
        match self.backends.get(&arch) {
            Some(backend) => backend(ir),
            None => panic!("Unsupported architecture: {:?}", arch),
        }
    }
}
