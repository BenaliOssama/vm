use vm::{blue, red};

use crate::arena::{self, Arena};
use crate::config::{CYCLE_DELTA, CYCLE_TO_DIE, MAX_CHECKS, NBR_LIVE};
use crate::helper;
use crate::player::Player;
use crate::process::Process;

// vm.rs
pub struct VirtualMachine {
    pub arena: Arena,
    pub processes: Vec<Process>,
    pub cycle_count: u64,
    pub cycles_to_die: usize,
    nbr_checks: usize,
    cycle_todie: usize,
}

impl VirtualMachine {
    pub fn new(arena: Arena, processes: Vec<Process>) -> Self {
        Self {
            arena,
            processes,
            cycle_count: 0,
            cycles_to_die: CYCLE_TO_DIE,
            nbr_checks: 0,
            cycle_todie: CYCLE_TO_DIE,
        }
    }

    pub fn load_player(&mut self, player: Player) {
        self.arena.write(0, &player.code);
        println!("{}", self.arena);
    }

    pub fn run(&mut self) {
        while self.processes_alive() {
            self.cycle();
            self.cycle_count += 1;
            if self.cycle_count % self.cycles_to_die as u64 == 0 {
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
        let mut child_process = vec![];
        let mut i = 0;
        for process in &mut self.processes {
            println!("{} {}", red("running process"), i);
            i += 1;
            let ch = process.execute_cycle(&mut self.arena);
            if ch.is_some() {
                println!("we found a pregrnant process");
                child_process.push(ch);
            }
        }
        if !child_process.is_empty() {
            for child in child_process {
                let mut c = child.unwrap();
                if c.current_instruction.is_none() {
                    continue;
                }
                let value = helper::get_value(
                    &c.current_instruction.clone().unwrap().parameters[0],
                    &c,
                    &self.arena,
                    true,
                );
                c.pc.set(value as usize, true);
                c.current_instruction = None;
                println!("{} {}", red("add process to vm at address: "), c.pc.get());
                self.processes.push(c);
            }
            println!("{} {:?}", red("current processes"), self.processes);
            println!("{}", self.arena);
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
