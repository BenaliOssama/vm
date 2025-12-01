//////////////////use corewar::{Arena, Process, VirtualMachine, parse_arguments};
use vm::*;

/*
the idea behine this test is to have specific compiled files (.cor) to run
and check the behaviour of the vm at certain steps.
this test should cover all instructions and make sure they do the job as should
 */
/*
## Instruction Testing Progress
- [x] live — announce that the process is alive
- [x] ld — load a value into a register
- [ ] st — store a register value into another register or memory
- [ ] add — add two registers and store the result
- [ ] sub — subtract two registers and store the result
- [ ] and — bitwise AND operation
- [ ] or — bitwise OR operation
- [ ] xor — bitwise XOR operation
- [z] zjmp — conditional jump if carry is set
- [ ] ldi — load from computed address into a register
- [ ] sti — store register value to computed address
- [ ] fork — duplicate a process at a relative address
- [ ] lld — long version of `ld` (no IDX_MOD)
- [ ] lldi — long version of `ldi` (no IDX_MOD)
- [ ] lfork — long version of `fork` (no IDX_MOD)
- [ ] nop — no operation (timing test instruction)
 */
#[test]
fn test_pierino_add() {
    // This simulates what main() does
    let args = vec!["vm".into(), "playground/players_src/pierino_add.cor".into()];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10 ld 5 ld 5 add 10
    for _ in 0..10 {
        vm.cycle();
    }
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    for _ in 0..5 {
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[2 - 1], 2);
    for _ in 0..5 {
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[3 - 1], 3);
    for _ in 0..10 {
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[4 - 1], 5);
    //ld 5 zjmp 20
    for _ in 0..20 {
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[3 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn test_pierino_and_ind_ind() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_and_ind_ind.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10 ld 5 ld 5 add 10
    for _ in 0..10 {
        vm.cycle();
    }
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //and
    for _ in 0..=6 {
        vm.cycle();
    }
    assert_eq!(vm.processes[0].pc.get(), 16);
    assert_eq!(vm.processes[0].registers[3 - 1], 0x302);
    // ld
    for _ in 0..=5 {
        vm.cycle();
    }
    assert_eq!(vm.processes[0].pc.get(), 21);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    //ld 5 zjmp 20
    for _ in 0..=20 {
        vm.cycle();
    }
    assert_eq!(vm.processes[0].pc.get(), 0);
}
