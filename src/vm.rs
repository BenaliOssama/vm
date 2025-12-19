//use vm::{blue, red};
use crate::arena::Arena;
use crate::config::{CYCLE_DELTA, CYCLE_TO_DIE, MAX_CHECKS, NBR_LIVE};
use crate::helper;
use crate::player::Player;
use crate::process::Process;
use crate::*;
use std::process as os;
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

    pub fn load_player(&mut self, player: Player, i: usize) {
        self.arena.write(i, &player.code);
        println!("{}", self.arena);
    }

    pub fn run(&mut self) {
        let mut cycles_since_last_check = 0; // 1. New accumulator
        while self.processes_alive() {
            for process in &mut self.processes {
                if process.state() == process::State::NoInstruction {
                    process.fetch_decode(&mut self.arena);
                }
            }
            self.debug1();
            self.cycle();
            // debugging lines goew here
            self.debug2();
            self.cycle_logic();
        }
    }
    pub fn cycle(&mut self) {
        let mut child_process = vec![];
        let mut i = 0;
        for process in &mut self.processes {
            //println!("{} {}", red("running process"), i);
            i += 1;
            let ch = process.execute_cycle(&mut self.arena, self.cycle_count);
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

    fn cycle_logic(&mut self) {
        // Every CYCLE_TO_DIE the VM will check every process and kill all the processes
        // that did not successfully execute any live instruction.

        // 2. Increment counters
        self.cycle_count += 1;

        if self.cycle_count >= self.cycles_to_die {
            println!("{} {}", vm::yellow("usual check: "), self.cycle_count);
            self.cycle_count = 0;

            self.check_lives();

            self.nbr_checks += 1;
            // To avoid infinite games, CYCLES_TO_DIE will be decremented by CYCLE_DELTA under certain conditions:
            //   - If during the last life loop there were at least NBR_LIVE successfully executed by the players.
            //   - If it has been MAX_CHECKS life loops since it was decremented last time.
            if self.read_nbr_lives() >= NBR_LIVE || self.nbr_checks >= MAX_CHECKS {
                self.cycle_to_die = self.cycle_to_die.checked_sub(CYCLE_DELTA).unwrap_or(0);
                //self.cycle_to_die = self.cycle_to_die - CYCLE_DELTA; //.checked_sub(CYCLE_DELTA).unwrap_or(0);
                if self.nbr_checks >= MAX_CHECKS {
                    self.nbr_checks = 0;
                }
                println!(
                    "{}  {}",
                    vm::green("reduce check cycle:"),
                    self.cycle_to_die
                );
            }
        }
        if self.cycle_to_die == 0 {
            println!("cycle to dies is 0");
            os::exit(0);
        }
    }

    fn debug1(&self) {
        println!(
            "{} ",
            green(
                "------------------------------------------------------------------------------------"
            )
        );
        println!(
            "Cycle {} || Cycles before life check: {} || Cycles between checks: {}",
            self.cycle_count,
            self.cycle_to_die.checked_sub(self.cycle_count).unwrap_or(0),
            self.cycle_to_die
        );

        println!("Processes:");
        println!("Id |Player Id |Pc   |Carry |Instr  |Wait |Registers");
        for p in self.processes.iter() {
            let current_instruction_name: String = if p.state() == process::State::Ready {
                "___".to_string()
            } else {
                p.current_instruction_name.clone()
            };
            print!(
                "{:>2} |{:>9} |{:>4} |{:5} |{:<6} |{:>4} | ",
                p.id,
                &p.player_id.to_string(),
                &p.instction_pc.to_string(),
                &p.carry.to_string(),
                current_instruction_name,
                &p.remaining_cycles.to_string()
            );

            // Registers print
            for (i, reg) in p.registers.iter().enumerate() {
                print!("{}:{:x}  ", i + 1, reg);
            }
            println!();
        }
    }
    fn debug2(&self) {
        println!("Players:");
        println!("Id |Last Live |Nb Live since last check");
        for pl in self.processes.iter() {
            println!(
                "{:>2} |{:>9} |{:>3}",
                pl.live_status.player_id, pl.live_status.last_live_cycle, pl.live_status.nbr_live
            );
        }

        println!("Arena:");
        let mut count = 0;
        for (i, byte) in self.arena.memory.iter().enumerate() {
            if i % 32 == 0 {
                print!("{:08x}  ", i);
            }
            print!("{:02x} ", byte);
            if i % 32 == 31 {
                println!("");
            }

            if count == 31 {
                break;
            }
            count += 1;
        }
        println!();
    }
    fn read_nbr_lives(&mut self) -> usize {
        let mut count = 0;
        for process in &mut self.processes {
            count += process.live_status.nbr_live;
            //process.live_status.nbr_live = 0;
        }
        count
    }

    fn rest_nbr_lives(&mut self) {
        for process in &mut self.processes {
            process.live_status.nbr_live = 0;
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
