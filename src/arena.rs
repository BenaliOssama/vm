// arena.rs
#[derive(Debug)]
pub struct Arena {
    memory: [u8; 4096],
}

impl Arena {
    pub fn new() -> Self {
        Self { memory: [0; 4096] }
    }
    pub fn write(&mut self, pos: usize, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.memory[(pos + i) % 4096] = byte;
        }
    }

    pub fn read(&self, pos: usize, size: usize) -> &[u8] {
        // Implement circular reading
        &self.memory[pos..pos + size]
    }
}
