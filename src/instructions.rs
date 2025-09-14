use crate::arena::*;
use crate::process::*;
// instruction.rs
#[derive(Debug, Clone)]
pub enum Parameter {
    Register(usize),
    Direct(i32),
    Indirect(i32),
}

#[derive(Debug, Clone)]
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
        process.carry = false;

        process.live_status.executed = true;

        if let Parameter::Direct(player_id) = self.parameters[0] {
            process.live_status.player_id = player_id;
        } else {
            eprintln!(
                "Invalid parameter for live instruction {:?}",
                self.parameters
            );
        }

        println!("heeeey!!! i'm alive :)");
    }
}
