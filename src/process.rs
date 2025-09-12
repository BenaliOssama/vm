use crate::arena::*;
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
#[derive(Clone)]
pub struct LiveStatus {
    pub executed: bool,
    pub player_id: i32, // negative of the player ID as per Core War convention
}

#[derive(Clone)]
pub struct Process {
    pub pc: PC, // Program Counter
    pub registers: [i32; 16],
    pub carry: bool,
    pub current_instruction: Option<Instruction>,
    pub remaining_cycles: i32,
    pub live_status: LiveStatus,
}

impl Process {
    pub fn new() -> Self {
        Self {
            pc: PC::new(),
            registers: [0; 16],
            remaining_cycles: 0,
            current_instruction: None,
            carry: false,
            live_status: LiveStatus {
                executed: false,
                player_id: 0,
            },
        }
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
    fn decode(&mut self, opcode: u8, raw_bytes: &[u8]) -> Instruction {
        let parameters = vec![]; // later: parse based on opcode/pcode

        let param = match opcode {
            1 => {
                //parameters
                let mut arr: [u8; 4] = raw_bytes.try_into().unwrap();
                let num = i32::from_be_bytes(arr);
                self.remaining_cycles = 10;
                Parameter::Direct(num)
            }
            _ => panic!("no paramiter"),
        };
        Instruction::new(opcode, parameters)
    }

    fn fetch_decode(&mut self, arena: &mut Arena) {
        let inst = arena.read(self.pc.get(), 1)[0];
        println!("address {} instruction : {:?}", self.pc.get(), inst);
        self.pc.add();
        if inst == 1 {
            let params = arena.read(self.pc.get(), 4);
            self.pc.set(self.pc.get() + 4);
            let inst = self.decode(inst, params);
            self.current_instruction = Some(inst);
        } else {
            println!("Not relevent for now");
            self.current_instruction = None;
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
                self.current_instruction
                    .take()
                    .unwrap()
                    .execute(self, arena);
            }
            State::NoInstruction => {
                println!("free...");
                self.fetch_decode(arena);
            }
        }
        thread::sleep(Duration::from_millis(400));
    }
}
