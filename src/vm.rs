use crate::arena::Arena;
use crate::player::Player;
use crate::process::Process;
// vm.rs
pub struct VirtualMachine {
    pub arena: Arena,
    pub processes: Vec<Process>,
    pub cycle_count: u64,
    pub cycles_to_die: i32,
    pub live_players: Vec<Player>,
}

impl VirtualMachine {
    pub fn new(arena: Arena, processes: Vec<Process>) -> Self {
        Self {
            arena,
            processes,
            cycle_count: 0,
            cycles_to_die: 10,
            live_players: vec![],
        }
    }

    pub fn load_player(&mut self, player: Player) {
        self.arena.write(0, &player.code);
    }

    pub fn run(&mut self) {
        while self.processes_alive() {
            self.cycle();
            self.cycle_count += 1;
            if self.cycle_count % self.cycles_to_die as u64 == 0 {
                self.check_lives();
            }
        }
    }

    fn cycle(&mut self) {
        for process in &mut self.processes {
            process.execute_cycle(&mut self.arena);
        }
        self.cycle_count += 1;
    }

    fn processes_alive(&self) -> bool {
        true
    }

    fn check_lives(&self) {
        // [ ] check lives
        for process in &self.processes {
            if process.live_status.executed {
                // [ ] add to live players after check
                //
            }
            // [ ] then reset process live
        }
    }

    fn kill_process() {
        // if vm.current_cycle - process.last_live_cycle > vm.cycle_to_die {
        //     // kill the process
        // }
    }
}
