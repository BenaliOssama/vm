mod arena;
mod instructions;
mod parser;
mod process;
mod utils;
mod vm;

pub use arena::*;
use parser::*;
use process::*;
use std::env;
use utils::*;

use crate::{parser::Player, vm::VirtualMachine};

fn main() {
    let args: Vec<String> = env::args().collect();

    let player = parse_arguments(args);
    let mut arena = Arena::new();
    let mut process = Process::new();
    let mut vm = VirtualMachine::new(arena, vec![process]);

    vm.load_player(player);
    vm.run();

    // // make new vm
    // let mut vm = VirtualMachine::new();

    // // add players to the memory
    // for player in players {
    //     vm.load_player(player);
    // }

    // // run the game
    // vm.run();
    // // end of the game, declare winner or no winner
    // vm.declare_winner();
}
