mod utils;

use vm::*;
use std::env;
use utils::*;

fn main() {
    println!("For this match the players will be:");
    // ./vm player1.cor player2.cor
    let args: Vec<String> = env::args().collect();

    println!("arguments are : {:?}", args);
    // get players in some vector of players
    let players = parse_arguments(args);

    println!("{} {:?}", green("we got players:"), players);
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
