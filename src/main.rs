mod arena;
mod instructions;
mod player;
mod process;
mod utils;
mod vm;

pub use arena::*;
use process::*;
use std::env;
use utils::*;
use vm::VirtualMachine;

fn main() {
    let args: Vec<String> = env::args().collect();

    let player = parse_arguments(args);
    let arena = Arena::new();
    let process = Process::new();

    let mut vm = VirtualMachine::new(arena, vec![process]);

    vm.load_player(player);
    vm.run();

    // end of the game, declare winner or no winner
    // vm.declare_winner();
}
