//use vm::{blue, red};
use crate::arena::Arena;
use crate::config::{CYCLE_DELTA, CYCLE_TO_DIE, MAX_CHECKS, NBR_LIVE};
use crate::helper;
use crate::instructions::VmAction;
use crate::player::Player;
use crate::process::Process;
use crate::*;
use std::collections::HashSet;
use std::hash::Hash;
//use std::process as os;
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
    pub winners: HashSet<i32>,
    pub cycle_count: usize,
    pub cycles_to_die: usize,
    nbr_checks: usize,
    cycles_since_check: usize,
    pub players: Vec<Player>,
    cycles_to_stop: i32,
}

impl VirtualMachine {
    pub fn create(
        arena: Arena,
        processes: Vec<Process>,
        players: Vec<Player>,
        cycles_to_stop: i32,
    ) -> Self {
        Self {
            arena,
            processes,
            cycle_count: 1, // fix this
            cycles_to_die: CYCLE_TO_DIE,
            nbr_checks: 0,
            cycles_since_check: 0,
            winners: HashSet::new(),
            players: players,
            cycles_to_stop: cycles_to_stop,
        }
    }

    pub fn load_player(&mut self, player: Player, i: usize) {
        self.arena.write(i, &player.code);
    }

    pub fn get_player(&self, id: i32) -> Option<String> {
        for player in &self.players {
            if player.id == id {
                return Some(player.name.clone());
            }
        }
        return None;
    }
    pub fn run(&mut self) {
        while self.processes_alive() {
            if self.cycles_to_stop > -1 && self.cycle_count as i32 >= self.cycles_to_stop {
                break;
            }
            for process in &mut self.processes {
                if process.state() == process::State::NoInstruction {
                    process.fetch_decode(&mut self.arena, self.cycle_count);
                }
            }
            //self.simple_debug();
            self.debug1();
            let before = self.cycles_to_die;
            let decreased = self.cycle_logic();
            self.cycle();
            // this is for convinience to look exactly like the reference vm giving.
            // otherwise it is not important to do the printing before the cycle or after!
            if decreased {
                println!(
                    "cycle {}: Cycles to die decreased: {} -> {}",
                    self.cycle_count, before, self.cycles_to_die
                );
            }
            // debugging lines goew here
            self.debug2();
            self.cycle_count += 1;
        }

        if self.winners.len() != 1 {
            println!("cycle {}: Nobody wins!", self.cycle_count);
        } else {
            let winner = *self.winners.iter().next().unwrap();

            let name = match self.get_player(winner) {
                Some(name) => name,
                None => "___".into(),
            };
            println!(
                "cycle {}: The winner is player ({}): {}!",
                //winner.live_status.last_live_cycle,
                self.cycle_count,
                winner * -1,
                name
            );
        }
    }
    // fn simple_debug(&self, process: &mut Process, current_cyle: usize) {
    // }
    pub fn cycle(&mut self) {
        let mut new_processes = Vec::new();
        for process in &mut self.processes {
            let action = process.execute_cycle(&mut self.arena, self.cycle_count);
            match action {
                VmAction::Fork { new_pc, use_idx } => {
                    let mut new_process =
                        process::Process::new(process.player_id, process.id, process.pc.get());
                    new_process.pc.set(new_pc as usize, use_idx);
                    new_process.current_instruction = None;
                    new_processes.push(new_process);
                }
                VmAction::Live(id) => {
                    match get_playername(self.players.clone(), id) {
                        Some(name) => {
                            process.live_status.executed = true;
                            process.live_status.nbr_live += 1;
                            process.live_status.last_live_cycle = self.cycle_count;
                            process.live_status.player_id = id;
                            println!(
                                "cycle {}: Player {} {} is alive",
                                self.cycle_count,
                                id * -1,
                                name
                            );
                        }
                        None => {
                            println!("cycle {}: live: Invalid argument: {}", self.cycle_count, id);
                        }
                    };
                }
                _ => {}
            }
        }
        // Append all new processes at once after the loop
        self.processes.extend(new_processes);
    }
    pub fn cycle_logic(&mut self) -> bool {
        let mut decreased = false;

        self.cycles_since_check += 1;

        if self.cycles_since_check > self.cycles_to_die {
            self.cycles_since_check = 0;

            self.check_lives();
            let nbr_lives = self.read_nbr_lives();

            // MAX_CHECKS logic
            if nbr_lives < NBR_LIVE {
                self.nbr_checks += 1;
            }

            // CTD decrease conditions
            if nbr_lives >= NBR_LIVE || self.nbr_checks > MAX_CHECKS {
                self.cycles_to_die = self.cycles_to_die.saturating_sub(CYCLE_DELTA);
                self.nbr_checks = 0;
                decreased = true;
            }

            self.rest_nbr_lives();
        }

        decreased
    }

    // pub fn cycle_logic(&mut self) -> bool {
    //     let mut decreased = false;
    //     // DO NOT increment cycle_count here
    //     self.cycles_since_check += 1;

    //     if self.cycles_since_check > self.cycles_to_die {
    //         self.cycles_since_check = 0;

    //         self.check_lives();

    //         let nbr_lives = self.read_nbr_lives();

    //         if nbr_lives >= NBR_LIVE {
    //             self.cycles_to_die = self.cycles_to_die.saturating_sub(CYCLE_DELTA);
    //             decreased = true;
    //             self.nbr_checks = 0;
    //         } else {
    //             self.nbr_checks += 1;
    //             if self.nbr_checks > MAX_CHECKS {
    //                 self.cycles_to_die = self.cycles_to_die.saturating_sub(CYCLE_DELTA);
    //                 decreased = true;
    //                 self.nbr_checks = 0;
    //             }
    //         }

    //         self.rest_nbr_lives();
    //     }
    //     // if self.cycles_to_die == 0 {
    //     //     os::exit(0);
    //     // }
    //     return decreased;
    // }

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
            self.cycles_to_die - self.cycles_since_check,
            self.cycles_to_die,
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

            // Registers //print
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
        }
        count
    }

    fn rest_nbr_lives(&mut self) {
        for process in &mut self.processes {
            process.live_status.nbr_live = 0;
        }
    }
    pub fn processes_alive(&self) -> bool {
        self.processes.len() > 0
    }

    fn check_lives(&mut self) {
        let mut winners = HashSet::new();
        for process in &self.processes {
            if self.get_player(process.live_status.player_id).is_some() {
                winners.insert(process.live_status.player_id);
            }
        }
        self.winners = winners;
        self.processes
            .retain(|process| process.live_status.executed);
        for process in &mut self.processes {
            process.live_status.executed = false;
        }
    }
}

fn get_playername(players: Vec<Player>, id: i32) -> Option<String> {
    for player in players {
        if player.id == id {
            return Some(player.name.clone());
        }
    }
    return None;
}
