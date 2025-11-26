//////////////////use corewar::{Arena, Process, VirtualMachine, parse_arguments};
use vm::*;

/*
the idea behine this test is to have specific compiled files (.cor) to run
and check the behaviour of the vm at certain steps.
this test should cover all instructions and make sure they do the job as should
 */
#[test]
fn test_vm_initialization_and_execution() {
    // This simulates what main() does
    let args = vec!["vm".into(), "playground/players_src/live.cor".into()];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::new(arena.clone(), vec![process]);

    vm.load_player(player);
    for _ in 0..50 {
        vm.cycle();
    }

}
