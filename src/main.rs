// use process::*;
use std::env;
use vm::*;
// use utils::*;
// use vm::VirtualMachine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let player = parse_arguments(args)?;
    let arena = Arena::new();
    let process = Process::new(player.id);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::new(arena.clone(), vec![process]);

    vm.load_player(player);
    vm.run();
    Ok(())
    // end of the game, declare winner or no winner
    // vm.declare_winner();
}
