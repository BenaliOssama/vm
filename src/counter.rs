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
        self.set((self.get() + size) as i32, false)
    }

    pub fn jump(&mut self, size: usize, use_idx_mod: bool) {
        self.set(size as i32, use_idx_mod); // size is relative to current PC
    }

    pub fn set(&mut self, offset: i32, use_idx_mod: bool) {
        let offset = if use_idx_mod {
            offset % IDX_MOD as i32
        } else {
            offset
        };

        let mut new_addr = (self.addr as i32 + offset) % MEM_SIZE as i32;
        if new_addr < 0 {
            new_addr += MEM_SIZE as i32;
        }

        self.addr = new_addr as usize;
    }

    pub fn get(&self) -> usize {
        return self.addr;
    }
}
