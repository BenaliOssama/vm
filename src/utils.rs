use vm::*;

pub fn parse_arguments(args: Vec<String>) -> Vec<String> {
    if args.len() < 3 {
        panic!("{}", red("not enought argumentsh"));
    }
    //TODO!
    return args[1..].to_vec();
}

