mod arena;
mod instructions;
mod parser;
mod process;
mod utils;

pub use arena::*;
use parser::*;
use process::*;
use std::env;
use utils::*;

use crate::parser::Player;

fn main() {
    let args: Vec<String> = env::args().collect();

    // get players in some vector of players
    let player = parse_arguments(args);

    let mut arena = Arena::new();

    arena.write(0, &player.code);

    let mut process1 = Process::new();
    process1.execute_cycle(&mut arena);

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
