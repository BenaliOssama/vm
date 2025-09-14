use crate::arena::Arena;
use crate::config::{CYCLE_DELTA, CYCLE_TO_DIE, MAX_CHECKS, NBR_LIVE};
use crate::player::Player;
use crate::process::Process;
// vm.rs
pub struct VirtualMachine {
    pub arena: Arena,
    pub processes: Vec<Process>,
    pub cycle_count: u64,
    pub cycles_to_die: i32,
    nbr_checks: usize,
    cycle_todie: usize,
}

impl VirtualMachine {
    pub fn new(arena: Arena, processes: Vec<Process>) -> Self {
        Self {
            arena,
            processes,
            cycle_count: 0,
            cycles_to_die: 10,
            nbr_checks: 0,
            cycle_todie: CYCLE_TO_DIE,
        }
    }

    pub fn load_player(&mut self, player: Player) {
        self.arena.write(0, &player.code);
    }

    pub fn run(&mut self) {
        while self.processes_alive() {
            self.cycle();
            self.cycle_count += 1;
            if self.cycle_count % CYCLE_TO_DIE as u64 == 0 {
                println!("{} {}", vm::yellow("usual check: "), self.cycle_count);
                self.check_lives();
                self.nbr_checks += 1;
                if self.read_nbr_lives() >= NBR_LIVE || self.nbr_checks % MAX_CHECKS == 0 {
                    self.cycle_todie -= CYCLE_DELTA;
                    self.nbr_checks = 0;
                    println!("{}  {}", vm::green("reduce check cycle:"), self.cycle_todie);
                }
            }
        }
    }
    fn read_nbr_lives(&mut self) -> usize {
        let mut count = 0;
        for process in &mut self.processes {
            count += process.live_status.nbr_live;
            process.live_status.nbr_live = 0;
        }
        count
    }
    fn cycle(&mut self) {
        for process in &mut self.processes {
            process.execute_cycle(&mut self.arena);
        }
    }

    fn processes_alive(&self) -> bool {
        self.processes.len() > 0
    }

    fn check_lives(&mut self) {
        self.processes
            .retain(|process| process.live_status.executed);
        for process in &mut self.processes {
            process.live_status.executed = false;
        }
    }
}
