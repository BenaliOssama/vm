use crate::player::Player;
use std::fs::File;
use std::io::Read;
use vm::*;

pub fn parse_arguments(args: Vec<String>) -> Player {
    /*_____________read arguments___________________ */
    if args.len() != 2 {
        panic!("{}", red("not enought argumentsh!"));
    }
    let file_name = args[1].as_str();

    //TODO!
    // make sure the file ends with the right extention .cor
    if !file_name.ends_with(".cor") {
        panic!("{}", red("bad file extention!"));
    }

    let mut file = File::open(file_name).unwrap();

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    let mut prev = 0;
    let mut next = 4;

    let magic = &buffer[prev..next];
    // 128 + 4
    prev = next;
    next = next + 128;

    let name = std::str::from_utf8(&buffer[prev..next]).unwrap(); //.trim();
    let name: String = name.chars().filter(|&c| c != '\0').collect();

    prev = next + 4; // skip 4 bytes 
    next = prev + 4;

    let mut arr = [0u8; 4];
    arr.copy_from_slice(&buffer[prev..next]);
    let size = u32::from_be_bytes(arr);

    prev = next; // skip 4 bytes 
    next = prev + 2048;

    let disc = std::str::from_utf8(&buffer[prev..next]).unwrap().trim();
    let disc: String = disc.chars().filter(|&c| c != '\0').collect();

    prev = next + 4; // skip 4 bytes 
    next = prev + size as usize;

    let program = &buffer[prev..next];

    let player = Player::new(
        -1,
        name.to_string(),
        disc.to_string(),
        program.to_vec(),
        size,
        0,
    );

    println!("{player}");
    return player;
}
