use crate::config::{IDX_MOD, MEM_SIZE};

#[derive(Debug, Clone)]
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
