// instruction.rs
pub enum Parameter {
    Register(usize),
    Direct(i32),
    Indirect(i32),
}

pub struct Instruction {
    opcode: u8,
    parameters: Vec<Parameter>,
    // ... other fields
}

impl Instruction {
    pub fn execute(&self, process: &mut Process, arena: &mut Arena) {
        match self.opcode {
            0x01 => self.live(process, arena),
            0x02 => self.ld(process, arena),
            // ... other instructions
            _ => panic!("Unknown instruction"),
        }
    }

    fn live(&self, process: &mut Process, arena: &mut Arena) {
        // Implement live instruction
    }
    // ... other instruction implementations
}
