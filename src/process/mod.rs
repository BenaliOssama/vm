pub mod decode;
pub mod display;

use crate::arena::*;
use crate::config::REG_NUMBER;
use crate::counter::PC;
use crate::instructions::*;

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
