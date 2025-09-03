// main.rs
fn main() {
    let args: Vec<String> = env::args().collect();
    let players = parse_arguments(&args);
    
    let mut vm = VirtualMachine::new();
    for player in players {
        vm.load_player(player);
    }
    
    vm.run();
    vm.declare_winner();
}
