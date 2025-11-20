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

// Unit tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_counter_set() {
        // no idx mod
        let mut pc = PC::new();
        pc.set(1, false);
        assert_eq!(pc.get(), 1);
        pc.set(MEM_SIZE - 1, false);
        assert_eq!(pc.get(), MEM_SIZE - 1);
        // with idx mod
        pc._reset();
        pc.set(1000, true);
        let should_be = 1000 % IDX_MOD;
        assert_eq!(pc.get(), should_be);
    }

    #[test]
    fn test_counter_reset() {
        let mut pc = PC::new();
        pc.add(1);
        pc._reset();
        assert_eq!(pc.get(), 0);
    }

    #[test]
    fn test_counter_inc() {
        let mut pc = PC::new();
        pc.inc();
        assert_eq!(pc.get(), 1);

        pc.add(MEM_SIZE - 2);
        assert_eq!(pc.get(), MEM_SIZE - 1);

        pc.inc();
        assert_eq!(pc.get(), 0);

        pc.inc();
        pc.inc();
        assert_eq!(pc.get(), 2);
    }

    #[test]
    fn test_counter_add() {
        let mut pc = PC::new();
        pc.add(1);
        assert_eq!(pc.get(), 1);
        pc.add(MEM_SIZE);
        assert_eq!(pc.get(), 1);
        pc.add(0);
        assert_eq!(pc.get(), 1);
    }

    #[test]
    fn test_couter_initialization() {
        let pc = PC::new();
        assert_eq!(pc.get(), 0);
    }
}
