use vm::*;

pub fn parse_arguments(args: Vec<String>) -> Vec<String> {
    if args.len() < 2 {
        panic!("{}", red("not enought argumentsh"));
    }
    //TODO!
    // make sure the file ends with the right extention .cor
    // for now i guess only one player is good. 
    return args[1..].to_vec();
}

