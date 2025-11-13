use crate::arena::{self, *};
use crate::config::MEM_SIZE;
use crate::helper;
use crate::process::*;
// instruction.rs
#[derive(Debug, Clone, Copy)]
pub enum Parameter {
    Register(usize),
    Direct(i32),
    Indirect(i32),
    None,
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
            2 => self.ld(process, arena),
            3 => self.st(process, arena),
            4 => self.add(process, arena),
            5 => self.sub(process, arena),
            _ => panic!("Unknown instruction"),
        }
    }

    fn live(&self, process: &mut Process, _arena: &mut Arena) {
        // Implement live instruction
        process.live_status.executed = true;
        process.live_status.nbr_live += 1;

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
    fn ld(&self, process: &mut Process, arena: &mut Arena) {
        let value = match self.parameters[0] {
            Parameter::Direct(v) | Parameter::Indirect(v) => v,
            Parameter::Indirect(v) => helper::bytes_to_i32(
                &arena.read(helper::wrap_address(process.pc.get(), v as i16), 4),
            ),
            _ => {
                eprintln!("Invalid first parameter for ld");
                return;
            }
        };
        let reg = match self.parameters[1] {
            Parameter::Register(r) => r,
            _ => {
                eprintln!("Invalid second parameter for ld");
                return;
            }
        };

        println!("ld: r{} ← {}", reg, value);
        process.registers[reg - 1] = value;
        println!("{}", process);
    }

    fn st(&self, process: &mut Process, arena: &mut Arena) {
        println!("{:?}", self.parameters);
        let source_reg = match self.parameters[0] {
            Parameter::Register(r) => r,
            _ => {
                eprintln!("Invalid second parameter for st");
                return;
            }
        };

        match self.parameters[1] {
            Parameter::Register(dist_reg) => {
                println!("st: r{} ← r{}", dist_reg, source_reg);
                process.registers[dist_reg - 1] = process.registers[source_reg - 1];
            }
            Parameter::Indirect(dist_memory) => {
                // pub fn write(&mut self, pos: usize, data: &[u8]) {
                println!(
                    "current address {} to be increased by {}",
                    process.pc.get(),
                    dist_memory
                );
                println!(
                    "st: m{} ← r{}",
                    (process.pc.get() + dist_memory as usize) % MEM_SIZE,
                    source_reg
                );
                arena.write(
                    (process.pc.get() + dist_memory as usize) % MEM_SIZE,
                    &process.registers[source_reg - 1].to_be_bytes(),
                );
                println!("{}", process);
            }
            _ => {
                eprintln!("Invalid first parameter for st");
                return;
            }
        };
        println!("{}", process);
        println!("{}", arena);
    }
    fn add(&self, process: &mut Process, arena: &mut Arena) {
        let reg1 = match self.parameters[0] {
            Parameter::Register(r) => r,
            _ => {
                eprintln!("Invalid second parameter for add");
                return;
            }
        };

        let reg2 = match self.parameters[1] {
            Parameter::Register(r) => r,
            _ => {
                eprintln!("Invalid second parameter for add");
                return;
            }
        };
        let reg3 = match self.parameters[2] {
            Parameter::Register(r) => r,
            _ => {
                eprintln!("Invalid second parameter for add");
                return;
            }
        };
        println!("add : r{} ← r{} + r{}", reg3, reg1, reg2);
        process.registers[reg3 - 1] = process.registers[reg1 - 1] + process.registers[reg2 - 1];
        println!("{}", process);
    }
    fn sub(&self, process: &mut Process, arena: &mut Arena) {
        let reg1 = match self.parameters[0] {
            Parameter::Register(r) => r,
            _ => {
                eprintln!("Invalid second parameter for add");
                return;
            }
        };

        let reg2 = match self.parameters[1] {
            Parameter::Register(r) => r,
            _ => {
                eprintln!("Invalid second parameter for add");
                return;
            }
        };
        let reg3 = match self.parameters[2] {
            Parameter::Register(r) => r,
            _ => {
                eprintln!("Invalid second parameter for add");
                return;
            }
        };
        println!("add : r{} ← r{} -jjjj r{}", reg3, reg1, reg2);
        process.registers[reg3 - 1] = process.registers[reg1 - 1] - process.registers[reg2 - 1];
        println!("{}", process);
    }
}

#[derive(Copy, Clone)]
pub struct InstructionInfo {
    pub nb_params: usize,
    pub nb_cycles: i32,
    pub has_pcode: bool,
    pub has_idx: bool,
    pub direct_size: usize, // 2 if IDX, 4 otherwise
}

pub const INSTRUCTION_TABLE: [InstructionInfo; 16] = [
    // 1. live
    InstructionInfo {
        nb_params: 1,
        nb_cycles: 10,
        has_pcode: false,
        has_idx: false,
        direct_size: 4,
    },
    // 2. ld
    InstructionInfo {
        nb_params: 2,
        nb_cycles: 5,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 3. st
    InstructionInfo {
        nb_params: 2,
        nb_cycles: 5,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 4. add
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 10,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 5. sub
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 10,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 6. and
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 6,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 7. or
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 6,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 8. xor
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 6,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 9. zjmp
    InstructionInfo {
        nb_params: 1,
        nb_cycles: 20,
        has_pcode: false,
        has_idx: true,
        direct_size: 2,
    },
    // 10. ldi
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 25,
        has_pcode: true,
        has_idx: true,
        direct_size: 2,
    },
    // 11. sti
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 25,
        has_pcode: true,
        has_idx: true,
        direct_size: 2,
    },
    // 12. fork
    InstructionInfo {
        nb_params: 1,
        nb_cycles: 800,
        has_pcode: false,
        has_idx: true,
        direct_size: 2,
    },
    // 13. lld
    InstructionInfo {
        nb_params: 2,
        nb_cycles: 10,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 14. lldi
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 50,
        has_pcode: true,
        has_idx: true,
        direct_size: 2,
    },
    // 15. lfork
    InstructionInfo {
        nb_params: 1,
        nb_cycles: 1000,
        has_pcode: false,
        has_idx: true,
        direct_size: 2,
    },
    // 16. nop
    InstructionInfo {
        nb_params: 1,
        nb_cycles: 2,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
];
