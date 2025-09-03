// vm.rs
pub struct VirtualMachine {
    pub arena: Arena,
    pub processes: Vec<Process>,
    pub cycle_count: u64,
    pub cycles_to_die: i32,
    // ... other fields
}

impl VirtualMachine {
    pub fn load_player(&mut self, player: Player) {
        // Load player code into arena
    }

    pub fn run(&mut self) {
        while self.processes_alive() {
            self.cycle();
            if self.cycle_count % self.cycles_to_die as u64 == 0 {
                self.check_lives();
            }
        }
    }

    fn cycle(&mut self) {
        for process in &mut self.processes {
            process.execute_cycle(&self.arena);
        }
        self.cycle_count += 1;
    }
}
