use std::fs::File;
use std::io::{self, Read};
use vm::*;

pub fn parse_arguments(args: Vec<String>) -> Vec<u8> {
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

    return buffer;
}
