//////////////////use corewar::{Arena, Process, VirtualMachine, parse_arguments};
use vm::*;

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
    vm.run();
    // let arena = Arena::new();
    // let process = Process::new(player.id);

    // let mut vm = VirtualMachine::new(arena.clone(), vec![process]);
    // vm.load_player(player);

    // // Run only N cycles if you have run_step()
    // for _ in 0..50 {
    //     vm.run_cycle(); // or vm.run_step()
    // }

    // // Now check state like you suggested
    // assert!(vm.processes.len() > 0);
    // assert_eq!(vm.processes[0].carry, true);
    // assert!(vm.last_live_id.is_some());
}
