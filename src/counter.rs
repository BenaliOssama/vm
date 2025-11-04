use crate::config::{IDX_MOD, MEM_SIZE};

#[derive(Debug, Clone)]
pub struct PC {
    pub addr: usize,
}

impl PC {
    pub fn new() -> Self {
        Self { addr: 0 }
    }

    pub fn inc(&mut self) {
        self.addr = (self.addr + 1) % MEM_SIZE;
    }

    pub fn _reset(&mut self) {
        self.addr = 0;
    }

    pub fn add(&mut self, size: usize) {
        self.set(self.get() + size, false)
    }

    pub fn set(&mut self, new_addr: usize, use_idx_mod: bool) {
        let addr = if use_idx_mod {
            new_addr % IDX_MOD
        } else {
            new_addr
        };
        self.addr = addr % MEM_SIZE; // always wrap around arena
    }

    pub fn get(&self) -> usize {
        return self.addr;
    }
}
