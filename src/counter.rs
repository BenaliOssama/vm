use crate::config::MEM_SIZE;

#[derive(Clone)]
pub struct PC {
    pub addr: usize,
}

impl PC {
    pub fn new() -> Self {
        Self { addr: 0 }
    }

    pub fn add(&mut self) {
        self.addr = (self.addr + 1) % MEM_SIZE;
    }

    pub fn reset(&mut self) {
        self.addr = 0;
    }

    pub fn set(&mut self, new_addr: usize) {
        self.addr = new_addr % MEM_SIZE;
    }

    pub fn get(&self) -> usize {
        return self.addr;
    }
}
