// use process::*;
use std::env;
use vm::*;
// use utils::*;
// use vm::VirtualMachine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The OS initialize the stack with arguments;
    // specifically, it will fill in the parameters to
    // the main() function, i.e., argc and the argv array.
    let args: Vec<String> = env::args().collect();

    let player = parse_arguments(args)?;
    let arena = Arena::new();
    // the loading process is done eagerly as old days
    // To understand how lazy loading of pieces of code and data works,
    // you’ll have to understand the machinery of paging and swapping,
    let process = Process::new(player.id);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    vm.run();
    Ok(())
    // end of the game, declare winner or no winner
    // vm.declare_winner();
}
