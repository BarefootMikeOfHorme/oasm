#![allow(dead_code)]
//! Instruction simulator
#[derive(Debug)]
pub struct CPU {
    pub pc: usize,
    pub regs: [i32; 8],
}

impl CPU {
    pub fn new() -> Self {
        Self { pc: 0, regs: [0; 8] }
    }

    pub fn step(&mut self, instr: &str) {
        println!("Executing instruction: {instr}");
        self.pc += 1;
    }

    pub fn run(&mut self, program: &[&str]) {
        for instr in program {
            self.step(instr);
        }
        println!("Program finished at PC={}", self.pc);
    }
}
