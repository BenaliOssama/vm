use crate::arena::*;
use crate::instructions::*;
use std::{thread, time::Duration};

// process.rs
pub struct Process {
    pub pc: usize, // Program Counter
    pub registers: [i32; 16],
    // pub carry: bool,
    pub last_live_cycle: i32,
    // pub alive: bool,
    // ... other fields
}

impl Process {
    pub fn new() -> Self {
        Self {
            pc: 0,
            registers: [0; 16],
            last_live_cycle: -1,
            // carry: false,
            // alive: true,
        }
    }

    fn fetch() {}

    fn decode(&self, opcode: u8, raw_bytes: &[u8]) -> Instruction {
        let parameters = vec![]; // later: parse based on opcode/pcode

        let param = match opcode {
            1 => {
                //parameters
                let mut arr: [u8; 4] = raw_bytes.try_into().unwrap();
                let num = i32::from_be_bytes(arr);
                Parameter::Direct(num)
            }
            _ => panic!("no paramiter"),
        };
        Instruction::new(opcode, parameters)
    }

    //Opcode ->
    pub fn execute_cycle(&mut self, arena: &mut Arena) {
        // Fetch and execute instruction
        loop {
            /*____________________________fetch__________________________ */
            // let's consider this to be fetching
            let inst = arena.read(self.pc, 1)[0];
            println!("address {} instruction : {:?}", self.pc, inst);
            self.pc += 1;
            thread::sleep(Duration::from_millis(1000));

            /*____________________________decode__________________________ */
            // https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/?
            // https://corewar-docs.readthedocs.io/en/latest/redcode/parser/
            // work on decoding an instruction
            // [Opcode] [Pcode?] [Param1] [Param2] [Param3]
            if inst == 1 {
                let params = arena.read(self.pc, 4);
                self.pc += 4;
                let inst = self.decode(inst, params);
                inst.execute(self, arena);
            } else {
                println!("Not relevent for now");
            }
            /*____________________________execute__________________________ */
        }
    }
}
