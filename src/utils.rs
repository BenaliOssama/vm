use crate::player::Player;
use crate::*;
use std::fs::File;
use std::io::Read;
use std::process;
pub fn parse_arguments(args: Vec<String>) -> Result<(Vec<Player>, i32, bool), String> {
    /*_____________read arguments___________________ */
    // Validation: Ensure at least one argument exists
    if args.len() < 2 {
        eprintln!("Usage: {} [-d N] [-v] <file1.cor> [file2.cor ...]", args[0]);
        std::process::exit(1);
    }

    let mut players = vec![];
    // --- NEW: Variables for flags ---
    let mut dump_cycles: i32 = -1; // Default to -1 (disabled)
    let mut visual_mode = false;
    let mut i = 1;

    // --- CHANGED: Use a while loop to control the index 'i' ---
    while i < args.len() {
        let arg = args[i].as_str();

        // Check for -d flag
        if arg == "-d" {
            if i + 1 >= args.len() {
                return Err(red("Flag -d requires a number argument"));
            }
            dump_cycles = args[i + 1]
                .parse::<i32>()
                .map_err(|_| red("Invalid number provided for -d"))?;
            i += 2; // Skip both "-d" and the number
            continue;
        }

        // Check for -v flag
        if arg == "-v" {
            visual_mode = true;
            i += 1;
            continue;
        }

        // --- EXISTING LOGIC: Treat as file ---
        let file_name = arg;

        // make sure the file ends with the right extention .cor
        if !file_name.ends_with(".cor") {
            panic!("{}", red("bad file extention!"));
        }

        let mut file =
            File::open(file_name).map_err(|e| red(&format!("Error Opening the file, {e}")))?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)
            .map_err(|e| red(&format!("Error reading the file, {e}")))?;

        let mut prev = 0;
        let mut next = 4;

        let magic = &buffer[prev..next];
        if magic != [0x00, 0xea, 0x83, 0xf3] {
            return Err(format!("invalid magic number: {:?}", magic));
        }

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

        // Don't forget to increment for the file loop
        i += 1;
    }

    // --- NEW: Validate player count at the end ---
    if players.len() > 4 || players.is_empty() {
        return Err(red("Invalid number of players. Must be between 1 and 4."));
    }

    // players.reverse();

    // --- CHANGED: Return the flags as well ---
    return Ok((players, dump_cycles, visual_mode));
}
// pub fn parse_arguments(args: Vec<String>) -> Result<Vec<Player>, String> {
//     /*_____________read arguments___________________ */
//     if args.len() < 2 || args.len() > 5 {
//         eprintln!("Usage: {} <file1.cor> [file2.cor ... up to 4]", args[0]);
//         process::exit(1); // exit with an error code
//     }
//     let mut players = vec![];

//     for i in 1..args.len() {
//         let file_name = args[i].as_str();
//         //TODO!
//         // make sure the file ends with the right extention .cor
//         if !file_name.ends_with(".cor") {
//             panic!("{}", red("bad file extention!"));
//         }

//         let mut file =
//             File::open(file_name).map_err(|e| red(&format!("Error Opening the file, {e}")))?;

//         let mut buffer = Vec::new();

//         file.read_to_end(&mut buffer)
//             .map_err(|e| red(&format!("Error reading the file, {e}")))?;

//         // if buffer.len() < config::HEADERS_SIZE {
//         //     return Err(red("the file are too smaaaaal"));
//         // }

//         let mut prev = 0;
//         let mut next = 4;

//         let magic = &buffer[prev..next];
//         if magic != [0x00, 0xea, 0x83, 0xf3] {
//             ////println!("invalid magic number: {:?}", magic);
//             return Err(format!("invalid magic number: {:?}", magic));
//         }
//         // 128 + 4
//         prev = next;
//         next = next + 128;

//         let name = std::str::from_utf8(&buffer[prev..next])
//             .map_err(|e| red(&format!("small file {e}")))?;
//         let name: String = name.chars().filter(|&c| c != '\0').collect();

//         prev = next + 4; // skip 4 bytes
//         next = prev + 4;

//         let mut arr = [0u8; 4];
//         arr.copy_from_slice(&buffer[prev..next]);
//         let size = u32::from_be_bytes(arr);
//         if size as usize > config::PLAYER_MAX_SIZE {
//             return Err(red("the file size are too big"));
//         }
//         prev = next; // skip 4 bytes
//         next = prev + 2048;

//         let disc = std::str::from_utf8(&buffer[prev..next])
//             .map_err(|e| red(&format!("small file {e}")))?
//             .trim();
//         let disc: String = disc.chars().filter(|&c| c != '\0').collect();

//         prev = next + 4; // skip 4 bytes
//         next = prev + size as usize;

//         let program = &buffer[prev..next];
//         if program.len() != size as usize {
//             return Err(red("the size is the header not the actual program size "));
//         }
//         let player = Player::new(
//             -1, //todo!()
//             name.to_string(),
//             disc.to_string(),
//             program.to_vec(),
//             size,
//             0,
//         );
//         players.push(player);
//     }
//     players.reverse();
//     return Ok(players);
// }
