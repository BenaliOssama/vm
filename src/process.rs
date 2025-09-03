// process.rs
pub struct Process {
    pub pc: usize,            // Program Counter
    pub registers: [i32; REG_NUMBER],
    pub carry: bool,
    pub alive: bool,
    // ... other fields
}

impl Process {
    pub fn execute_cycle(&mut self, arena: &Arena) {
        // Fetch and execute instruction
    }
}
