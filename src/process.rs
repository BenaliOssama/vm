use crate::arena::*;
// process.rs
pub struct Process {
    pub pc: usize, // Program Counter
                   // pub registers: [i32; 8],
                   // pub carry: bool,
                   // pub alive: bool,
                   // ... other fields
}

impl Process {
    pub fn new() -> Self {
        Self {
            pc: 0,
            // registers: [0; 8],
            // carry: false,
            // alive: true,
        }
    }

    pub fn execute_cycle(&mut self, arena: &Arena) {
        // Fetch and execute instruction
        let inst = arena.read(0, 1);
        println!("this is the instruction fetched: {:?}", inst);
    }
}
