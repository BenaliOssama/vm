use crate::config::{MEM_SIZE, REG_NUMBER};
use crate::counter::PC;
use crate::{arena, instructions::*};
use crate::{arena::*, instructions};
use std::fmt::Display;
use std::fmt::Formatter;
use vm::*;

use std::{thread, time::Duration};

// process.rs
// https://www.geeksforgeeks.org/operating-systems/process-in-operating-system/
// https://www.geeksforgeeks.org/operating-systems/process-control-block-in-os/
// running, waiting, or ready to execute.
enum State {
    Waiting,
    Ready,
    NoInstruction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamType {
    None,
    Register,
    Direct,
    Indirect,
}

#[derive(Debug, Clone)]
pub struct LiveStatus {
    pub executed: bool,
    pub player_id: i32,  // negative of the player ID as per Core War convention
    pub nbr_live: usize, // used with "Stop process execution"
}

#[derive(Debug, Clone)]
pub struct Process {
    pub pc: PC, // Program Counter
    pub registers: [i32; REG_NUMBER],
    pub carry: bool,
    pub current_instruction: Option<Instruction>,
    pub remaining_cycles: i32,
    pub live_status: LiveStatus,
}

impl Process {
    pub fn new(player: i32) -> Self {
        let mut pro = Self {
            pc: PC::new(),
            registers: [0; REG_NUMBER],
            remaining_cycles: 0,
            current_instruction: None,
            carry: false,
            live_status: LiveStatus {
                executed: false,
                player_id: 0,
                nbr_live: 0,
            },
        };
        pro.registers[0] = player;
        pro
    }
    fn state(&self) -> State {
        if self.current_instruction.is_some() && self.remaining_cycles == 0 {
            return State::Ready;
        } else if self.current_instruction.is_some() && self.remaining_cycles != 0 {
            return State::Waiting;
        } else {
            return State::NoInstruction;
        }
    }
    fn decode(&mut self, opcode: u8, arena: &mut Arena) -> Option<Instruction> {
        let inst_index = (opcode - 1) as usize;
        let inst_info = INSTRUCTION_TABLE[inst_index]; // instructions table is 1-indexed

        match opcode {
            // -------------------------------------------------------------------------
            // live %<direct>
            // -------------------------------------------------------------------------
            1 => {
                // live has no pcode, always 4-byte direct
                let bytes = arena.read(self.pc.get(), 4);
                self.pc.add(4);
                let value = bytes_to_i32(&bytes);

                self.remaining_cycles = inst_info.nb_cycles.saturating_sub(2);
                println!("{}: {}", vm::cyan("LIVE param"), value);

                Some(Instruction::new(opcode, vec![Parameter::Direct(value)]))
            }

            // -------------------------------------------------------------------------
            // ld <direct|indirect>, <register>
            // -------------------------------------------------------------------------
            2 => {
                // read and decode pcode
                let pcode = arena.read(self.pc.get(), 1)[0];
                self.pc.inc();

                let type_params = decode_pcode(pcode, inst_info.nb_params);

                // validate params
                let first_ok = matches!(
                    type_params.get(0),
                    Some(ParamType::Direct | ParamType::Indirect)
                );
                let second_ok = matches!(type_params.get(1), Some(ParamType::Register));
                if !first_ok || !second_ok {
                    eprintln!(
                        "Invalid parameter types for ld: {:?} {:?}",
                        type_params.get(0),
                        type_params.get(1)
                    );
                    return None;
                }

                // decode parameters
                let mut params = Vec::new();
                for param_type in type_params.iter() {
                    let param = match param_type {
                        ParamType::Direct => {
                            let size = if inst_info.has_idx { 2 } else { 4 };
                            let bytes = arena.read(self.pc.get(), size);
                            self.pc.add(size);
                            Parameter::Direct(bytes_to_i32(&bytes))
                        }
                        ParamType::Indirect => {
                            let bytes = arena.read(self.pc.get(), 2);
                            self.pc.add(2);
                            let offset = bytes_to_i16(&bytes);
                            let addr = wrap_address(self.pc.get(), offset);
                            let value = bytes_to_i32(&arena.read(addr, 4));
                            Parameter::Indirect(value)
                        }
                        ParamType::Register => {
                            let reg = arena.read(self.pc.get(), 1)[0] as usize;
                            self.pc.inc();
                            Parameter::Register(reg)
                        }
                        _ => Parameter::None,
                    };
                    params.push(param);
                }

                Some(Instruction::new(opcode, params))
            }

            // -------------------------------------------------------------------------
            _ => {
                eprintln!("Unknown opcode {}", opcode);
                None
            }
        }
    }

    fn fetch_decode(&mut self, arena: &mut Arena) {
        let opcode = arena.read(self.pc.get(), 1)[0];
        self.pc.inc();
        println!("address {} instruction : {:?}", self.pc.get(), opcode);
        match opcode {
            // [ ] i must verify the integrety of the arguments, if currepted i jump.
            1 => {
                println!("{}", vm::blue("LIVE"));
                let inst = self.decode(opcode, arena);
                self.current_instruction = inst;
            }
            2 => {
                println!("{}", vm::blue("LD"));
                let inst = self.decode(opcode, arena);
                self.current_instruction = inst;
            }
            _ => {
                println!("Not relevent for now");
                self.current_instruction = None;
            }
        }
    }
    //Opcode ->
    // https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/?
    // https://corewar-docs.readthedocs.io/en/latest/redcode/parser/
    // work on decoding an instruction
    // [Opcode] [Pcode?] [Param1] [Param2] [Param3]
    pub fn execute_cycle(&mut self, arena: &mut Arena) {
        match self.state() {
            State::Waiting => {
                println!("waiting...");
                self.remaining_cycles -= 1;
            }
            State::Ready => {
                println!("executing...");
                println!("instruction {:?}", self.current_instruction);
                self.current_instruction
                    .take()
                    .unwrap()
                    .execute(self, arena);
                // self.live_status.nbr_live += 1;
            }
            State::NoInstruction => {
                println!("free...");
                self.fetch_decode(arena);
            }
        }
        thread::sleep(Duration::from_millis(60));
    }
}

impl Display for Process {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // create a table
        let mut table = Table::new();

        // add headers
        table.add_header("PC");
        table.add_header("Carry");
        table.add_header("Current Instruction");
        table.add_header("Remaining Cycles");
        table.add_header("Lives Status");
        table.add_header("Registers");

        let registers_str = self
            .registers
            .iter()
            .enumerate()
            .map(|(i, val)| format!("R{}:{}", i + 1, val))
            .collect::<Vec<_>>()
            .join(", ");

        // prepare live status as string
        let live_status_str = format!(
            "executed: {}, player_id: {}, nbr_live: {}",
            self.live_status.executed, self.live_status.player_id, self.live_status.nbr_live
        );

        // prepare current instruction as string
        let current_inst_str = match &self.current_instruction {
            Some(inst) => format!("{:?}", inst),
            None => "None".to_string(),
        };

        // add row with all fields converted to strings
        table.add_row(&vec![
            self.pc.get().to_string(),
            self.carry.to_string(),
            current_inst_str,
            self.remaining_cycles.to_string(),
            live_status_str,
            registers_str,
        ]);

        // print the table
        println!("{table}");

        Ok(())
    }
}

fn decode_pcode(pcode: u8, num_args: usize) -> [ParamType; 3] {
    let mut result = [ParamType::None; 3];

    for i in 0..num_args {
        let shift = 6 - (i * 2);
        let bits = (pcode >> shift) & 0b11;
        result[i] = match bits {
            0b01 => ParamType::Register,
            0b10 => ParamType::Direct,
            0b11 => ParamType::Indirect,
            _ => ParamType::None, // 0b00 means unused/invalid
        };
    }

    result
}

fn bytes_to_i32(bytes: &[u8]) -> i32 {
    let mut arr = [0u8; 4]; // 4 bytes for i32
    let len = bytes.len();
    // copy bytes to the end of the array (big-endian)
    arr[4 - len..].copy_from_slice(bytes);
    i32::from_be_bytes(arr)
}

fn bytes_to_i16(bytes: &[u8]) -> i16 {
    let mut arr = [0u8; 2]; // 2 bytes for i16
    let len = bytes.len();
    arr[2 - len..].copy_from_slice(bytes);
    i16::from_be_bytes(arr)
}

fn wrap_address(pc: usize, offset: i16) -> usize {
    let mut addr = (pc as isize + offset as isize) % MEM_SIZE as isize;
    if addr < 0 {
        addr += MEM_SIZE as isize;
    }
    addr as usize
}
