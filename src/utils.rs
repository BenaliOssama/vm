use crate::player::Player;
use crate::*;
use std::fs::File;
use std::io::Read;
use std::process;

pub fn parse_arguments(args: Vec<String>) -> Result<Vec<Player>, String> {
    /*_____________read arguments___________________ */
    if args.len() < 2 || args.len() > 5 {
        eprintln!("Usage: {} <file1.cor> [file2.cor ... up to 4]", args[0]);
        process::exit(1); // exit with an error code
    }
    let mut players = vec![];

    for i in 1..args.len() {
        let file_name = args[i].as_str();

        //TODO!
        // make sure the file ends with the right extention .cor
        if !file_name.ends_with(".cor") {
            panic!("{}", red("bad file extention!"));
        }

        let mut file =
            File::open(file_name).map_err(|e| red(&format!("Error Opening the file, {e}")))?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)
            .map_err(|e| red(&format!("Error reading the file, {e}")))?;

        // if buffer.len() < config::HEADERS_SIZE {
        //     return Err(red("the file are too smaaaaal"));
        // }

        let mut prev = 0;
        let mut next = 4;

        let magic = &buffer[prev..next];
        if magic != [0x00, 0xea, 0x83, 0xf3] {
            println!("invalid magic number: {:?}", magic);
            return Err(format!("invalid magic number: {:?}", magic));
        }
        // 128 + 4
        prev = next;
        next = next + 128;

        let name = std::str::from_utf8(&buffer[prev..next])
            .map_err(|e| red(&format!("small file {e}")))?;
        let name: String = name.chars().filter(|&c| c != '\0').collect();

        prev = next + 4; // skip 4 bytes 
        next = prev + 4;

        let mut arr = [0u8; 4];
        arr.copy_from_slice(&buffer[prev..next]);
        let size = u32::from_be_bytes(arr);
        if size as usize > config::PLAYER_MAX_SIZE {
            return Err(red("the file size are too big"));
        }
        prev = next; // skip 4 bytes 
        next = prev + 2048;

        let disc = std::str::from_utf8(&buffer[prev..next])
            .map_err(|e| red(&format!("small file {e}")))?
            .trim();
        let disc: String = disc.chars().filter(|&c| c != '\0').collect();

        prev = next + 4; // skip 4 bytes 
        next = prev + size as usize;

        let program = &buffer[prev..next];
        if program.len() != size as usize {
            return Err(red("the size is the header not the actual program size "));
        }
        let player = Player::new(
            -1, //todo!()
            name.to_string(),
            disc.to_string(),
            program.to_vec(),
            size,
            0,
        );
        players.push(player);
    }
    players.reverse();
    return Ok(players);
}
