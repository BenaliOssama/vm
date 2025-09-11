use crate::arena::*;
use crate::process::*;
// instruction.rs
#[derive(Clone)]
pub enum Parameter {
    Register(usize),
    Direct(i32),
    Indirect(i32),
}
impl Parameter {
    pub fn new() {}
}
#[derive(Clone)]
pub struct Instruction {
    opcode: u8,
    parameters: Vec<Parameter>,
}

impl Instruction {
    pub fn new(opcode: u8, parameters: Vec<Parameter>) -> Self {
        Self { opcode, parameters }
    }

    pub fn execute(&self, process: &mut Process, arena: &mut Arena) {
        match self.opcode {
            1 => self.live(process, arena),
            // 0x02 => self.ld(process, arena),
            // // ... other instructions
            _ => panic!("Unknown instruction"),
        }
    }

    fn live(&self, process: &mut Process, arena: &mut Arena) {
        // Implement live instruction
        process.last_live_cycle += 1;
        process.carry = false;
        println!("heeeey!!! i'm alive :)");
    }
    // ... other instruction implementations
}
