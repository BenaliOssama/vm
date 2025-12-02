//use vm::{blue, red};
use crate::arena::{self, Arena};
use crate::config::{CYCLE_DELTA, CYCLE_TO_DIE, MAX_CHECKS, NBR_LIVE};
use crate::helper;
use crate::player::Player;
use crate::process::{Process, State};
use crate::*;
/*
[X] create
[ ] destroy
[ ] wait
[ ] miscellaneous
[ ] control (suspend)
[ ] status
 */

// vm.rs
pub struct VirtualMachine {
    pub arena: Arena,
    pub processes: Vec<Process>,
    pub cycle_count: usize,
    pub cycles_to_die: usize,
    nbr_checks: usize,
    cycle_to_die: usize,
}

impl VirtualMachine {
    pub fn create(arena: Arena, processes: Vec<Process>) -> Self {
        Self {
            arena,
            processes,
            cycle_count: 1,
            cycles_to_die: CYCLE_TO_DIE,
            nbr_checks: 0,
            cycle_to_die: CYCLE_TO_DIE,
        }
    }

    pub fn load_player(&mut self, player: Player) {
        self.arena.write(0, &player.code);
        println!("{}", self.arena);
    }

    pub fn run(&mut self) {
        while self.processes_alive() {
            self.cycle();
            // debugging lines goew here
            println!(
                "{} ",
                green(
                    "------------------------------------------------------------------------------------"
                )
            );
            println!(
                "Cycle {} || Cycles before life check: {} || Cycles between checks: {}",
                self.cycle_count,
                self.cycle_to_die - self.cycle_count,
                self.cycle_to_die
            );

            println!("Processes:");
            println!("Id |Player Id |Pc   |Carry |Instr  |Wait |Registers");
            for p in self.processes.iter() {
                print!(
                    "{:>2} |{:>9} |{:>4} |{:5} |{:<6} |{:>4} | ",
                    p.id,
                    &p.player_id.to_string(),
                    &p.instction_pc.to_string(),
                    &p.carry.to_string(),
                    p.current_instruction_name,
                    &p.remaining_cycles.to_string()
                );

                // Registers print
                for (i, reg) in p.registers.iter().enumerate() {
                    print!("{}:{:x}  ", i + 1, reg);
                }
                println!();
            }

            // println!("Players:");
            // println!("Id |Last Live |Nb Live since last check");
            // for pl in self.processes.iter() {
            //     println!(
            //         "{:>2} |{:>9} |{:>3}",
            //         pl.id, pl.last_live_cycle, pl.live_count_since_check
            //     );
            // }

            // println!("Arena:");
            // for (i, byte) in self.arena.iter().enumerate() {
            //     if i % 32 == 0 {
            //         print!("{:08x}  ", i);
            //     }
            //     print!("{:02x} ", byte);
            //     if i % 32 == 31 {
            //         println!();
            //     }
            // }
            // println!();

            println!(
                "{} ",
                green(
                    "------------------------------------------------------------------------------------"
                )
            );
            self.cycle_count += 1;
            if self.cycle_count % self.cycles_to_die == 0 {
                println!("{} {}", vm::yellow("usual check: "), self.cycle_count);
                self.check_lives();
                self.nbr_checks += 1;
                if self.read_nbr_lives() >= NBR_LIVE || self.nbr_checks % MAX_CHECKS == 0 {
                    self.cycle_to_die -= CYCLE_DELTA;
                    self.nbr_checks = 0;
                    println!(
                        "{}  {}",
                        vm::green("reduce check cycle:"),
                        self.cycle_to_die
                    );
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

    pub fn cycle(&mut self) {
        let mut child_process = vec![];
        let mut i = 0;
        for process in &mut self.processes {
            println!("{} {}", red("running process"), i);
            i += 1;
            if process.state() == process::State::NoInstruction {
                process.fetch_decode(&mut self.arena);
            }
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
                if c.current_instruction.clone().unwrap().opcode == 15 {
                    c.pc.set(value as usize, false);
                } else {
                    c.pc.set(value as usize, true);
                }
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
