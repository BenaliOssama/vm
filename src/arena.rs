// arena.rs
pub struct Arena {
    memory: [u8; MEM_SIZE],
}

impl Arena {
    pub fn write(&mut self, pos: usize, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.memory[(pos + i) % MEM_SIZE] = byte;
        }
    }

    pub fn read(&self, pos: usize, size: usize) -> &[u8] {
        // Implement circular reading
    }
}
