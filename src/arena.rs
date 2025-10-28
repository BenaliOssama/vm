use crate::config::MEM_SIZE;

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
    pub fn read(&self, pos: usize, size: usize) -> Vec<u8> {
        let mut arr = Vec::with_capacity(size);
        let mut current_pos = pos % MEM_SIZE;
        for _ in 0..size {
            arr.push(self.memory[current_pos]);
            current_pos = (current_pos + 1) % MEM_SIZE; // proper circular wrap
        }
        arr
    }
}
