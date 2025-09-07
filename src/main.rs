mod arena;
mod utils;

use arena::*;
use std::env;
use utils::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    // get players in some vector of players
    let buffer = parse_arguments(args);

    let mut prev = 0;
    let mut next = 4;

    let magic = &buffer[prev..next];
    // 128 + 4
    prev = next;
    next = next + 128;

    let name = std::str::from_utf8(&buffer[prev..next]).unwrap();
    // Print as uppercase hex
    for b in magic {
        print!("{:02x} ", b);
    }

    prev = next + 4; // skip 4 bytes 
    next = prev + 4;

    let mut arr = [0u8; 4];
    println!("this is what i read: {:?}", &buffer[prev..next]);
    arr.copy_from_slice(&buffer[prev..next]);
    println!("\nBytes for size: {:02x?}", arr);
    let size = u32::from_be_bytes(arr);

    prev = next; // skip 4 bytes 
    next = prev + 2048;

    let disc = std::str::from_utf8(&buffer[prev..next]).unwrap();
    // Print as uppercase hex
    for b in magic {
        print!("{:02x} ", b);
    }

    prev = next + 4; // skip 4 bytes 
    next = prev + size as usize;

    let program = &buffer[prev..next];

    println!(" -> should be -> {}", "00 ea 83 f3");
    println!("name -> {}", name);

    println!("size {:}", size);
    println!("description {:}", disc);
    for b in program {
        print!("{:02x} ", b);
    }
    println!();

    let mut arena = Arena::new();
    println!("empty arena: -> : {:?}", arena);

    arena.write(0, program);
    println!("fulll arena: -> : {:?}", arena);
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
