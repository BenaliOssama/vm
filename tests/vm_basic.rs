//////////////////use corewar::{Arena, Process, VirtualMachine, parse_arguments};
use vm::State;
use vm::*;
/*
the idea behine this test is to have specific compiled files (.cor) to run
and check the behaviour of the vm at certain steps.
this test should cover all instructions and make sure they do the job as should
 */
/*
## Instruction Testing Progress
- [X] live — announce that the process is alive
- [X] ld — load a value into a register
- [X] st — store a register value into another register or memory
- [X] add — add two registers and store the result
- [X] sub — subtract two registers and store the result
- [X] and — bitwise AND operation
- [X] or — bitwise OR operation
- [X] xor — bitwise XOR operation
- [X] zjmp — conditional jump if carry is set
- [X] ldi — load from computed address into a register
- [X] sti — store register value to computed address
- [X] fork — duplicate a process at a relative address
- [X] lld — long version of `ld` (no IDX_MOD)
- [ ] lldi — long version of `ldi` (no IDX_MOD)
- [ ] lfork — long version of `fork` (no IDX_MOD)
- [ ] nop — no operation (timing test instruction)
 */

#[test]
fn add() {
    // This simulates what main() does
    let args = vec!["vm".into(), "playground/players_src/pierino_add.cor".into()];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process); //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ld 5
    for _ in 0..5 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[2 - 1], 2);
    run_for(&mut vm, 5);
    assert_eq!(vm.processes[0].registers[3 - 1], 3);
    // add
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].registers[4 - 1], 5);
    run_for(&mut vm, 5);
    //ld 5 zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[3 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn lld_dir_reg() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_lld_dir_reg.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);

    //lld 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].registers[2 - 1], 1234); // ae4fffc

    //ld
    run_for(&mut vm, 5);
    //ld 5 zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn sti_reg_dir_dir() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_sti_reg_dir_dir.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    run_for(&mut vm, 5);
    //sti 25
    run_for(&mut vm, 25);
    //println!("{}", vm.arena);
    //01 ff ff ff ff 02 90 00 00 00 7b 02 ff ff ff ff
    //01 FF FF FF FF 02 90 00 00 00 7B 02 FF FF FF FF
    let at_mem = vm.arena.read(36, 4);
    assert_eq!(vec![255, 255, 255, 255], at_mem);
    run_for(&mut vm, 5);
    // zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn and_ind_ind() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_and_ind_ind.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //and 6
    run_for(&mut vm, 6);
    assert_eq!(vm.processes[0].registers[3 - 1], 770);
    run_for(&mut vm, 5);
    //ld 5 zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
#[ignore]
fn lldi_dir_dir_reg() {
    todo!()
}
#[test]
fn sti_reg_dir_reg() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_sti_reg_dir_reg.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    run_for(&mut vm, 5);
    //sti 25
    run_for(&mut vm, 25);
    //println!("{}", vm.arena);
    //01 ff ff ff ff 02 90 00 00 00 7b 02 ff ff ff ff
    //01 FF FF FF FF 02 90 00 00 00 7B 02 FF FF FF FF
    let at_mem = vm.arena.read(158, 4);
    assert_eq!(vec![255, 255, 255, 255], at_mem);
    run_for(&mut vm, 5);
    // zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
#[ignore]
fn and_ind_reg() {
    todo!()
}
#[test]
#[ignore]
fn lldi_dir_reg_reg() {
    todo!()
}

#[test]
fn sti_reg_ind_dir() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_sti_reg_ind_dir.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    run_for(&mut vm, 5);
    //sti 25
    run_for(&mut vm, 25);
    //println!("{}", vm.arena);
    //01 ff ff ff ff 02 90 00 00 00 7b 02 ff ff ff ff
    //01 FF FF FF FF 02 90 00 00 00 7B 02 FF FF FF FF
    let at_mem = vm.arena.read(12, 4);
    assert_eq!(vec![255, 255, 255, 255], at_mem);
    run_for(&mut vm, 5);
    // zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
#[ignore]
fn and_reg_ind() {
    todo!()
}

#[test]
fn lldi_ind_dir_reg() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_lldi_ind_dir_reg.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    println!("{}", vm.arena);
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //lldi 50
    run_for(&mut vm, 50);
    assert_eq!(vm.processes[0].registers[3 - 1], 133631); // 209ff //209FFED

    run_for(&mut vm, 5);
    //ld 5 zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn sti_reg_ind_reg() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_sti_reg_ind_reg.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    run_for(&mut vm, 5);
    //sti 25
    run_for(&mut vm, 25);
    //println!("{}", vm.arena);
    //01 ff ff ff ff 02 90 00 00 00 7b 02 ff ff ff ff
    //01 FF FF FF FF 02 90 00 00 00 7B 02 FF FF FF FF
    let at_mem = vm.arena.read(12, 4);
    assert_eq!(vec![255, 255, 255, 255], at_mem);
    run_for(&mut vm, 5);
    // zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
#[ignore]
fn and_reg_reg() {
    todo!()
}
#[test]
#[ignore]
fn lldi_ind_reg_reg() {
    todo!()
}
#[test]
fn sti_reg_reg_dir() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_sti_reg_reg_dir.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    run_for(&mut vm, 5);
    //sti 25
    run_for(&mut vm, 25);
    //println!("{}", vm.arena);
    //01 ff ff ff ff 02 90 00 00 00 7b 02 ff ff ff ff
    //01 FF FF FF FF 02 90 00 00 00 7B 02 FF FF FF FF
    let at_mem = vm.arena.read(136, 4);
    assert_eq!(vec![255, 255, 255, 255], at_mem);
    run_for(&mut vm, 5);
    // zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn fork() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_fork.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // fork 800
    run_for(&mut vm, 800);
    assert_eq!(vm.processes.len(), 2);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);

    assert_eq!(vm.processes[1].live_status.executed, true);
    assert_eq!(vm.processes[1].live_status.player_id, -1);
    assert_eq!(vm.processes[1].live_status.nbr_live, 1);

    run_for(&mut vm, 5);
    // zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 3);

    assert_eq!(vm.processes[1].registers[2 - 1], 0);
    assert_eq!(vm.processes[1].pc.get(), 3);
}

#[test]
fn lld_ind_reg() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_lld_ind_reg.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);

    //lld 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].registers[2 - 1], -1); // ae4fffc

    //ld
    run_for(&mut vm, 5);
    //ld 5 zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn sti_reg_reg_reg() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_sti_reg_reg_reg.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    run_for(&mut vm, 5);
    //sti 25
    run_for(&mut vm, 25);
    //println!("{}", vm.arena);
    //01 ff ff ff ff 02 90 00 00 00 7b 02 ff ff ff ff
    //01 FF FF FF FF 02 90 00 00 00 7B 02 FF FF FF FF
    let at_mem = vm.arena.read(135, 4);
    assert_eq!(vec![255, 255, 255, 255], at_mem);
    run_for(&mut vm, 5);
    // zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn ldi_dir_dir() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_ldi_dir_dir.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ldi 25
    run_for(&mut vm, 25);
    assert_eq!(vm.processes[0].registers[3 - 1], 65537); // ae4fffc
    // ld
    run_for(&mut vm, 5);
    //zjmp
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
#[ignore]
fn lldi_reg_dir_reg() {
    todo!()
}
#[test]
#[ignore]
fn st_reg() {
    todo!()
}
#[test]
fn ldi_dir_reg() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_ldi_dir_reg.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    // ld
    run_for(&mut vm, 5);
    //ldi 25
    run_for(&mut vm, 25);
    assert_eq!(vm.processes[0].registers[3 - 1], 133631); // ae4fffc
    // ld
    run_for(&mut vm, 5);
    //zjmp
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
#[ignore]
fn lldi_reg_reg_reg() {
    todo!()
}

#[test]
fn sub() {
    // This simulates what main() does
    let args = vec!["vm".into(), "playground/players_src/pierino_sub.cor".into()];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ld 5
    for _ in 0..5 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[2 - 1], 2);
    run_for(&mut vm, 5);
    assert_eq!(vm.processes[0].registers[3 - 1], 3);
    // sub
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].registers[4 - 1], -1);
    run_for(&mut vm, 5);
    //ld 5 zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn ldi_ind_dir() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_ldi_ind_dir.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ldi 25
    run_for(&mut vm, 25);
    assert_eq!(vm.processes[0].registers[3 - 1], 182779900); // ae4fffc
    run_for(&mut vm, 5);
    //ld 5 zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn or_ind_ind() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_or_ind_ind.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //or 6
    run_for(&mut vm, 6);
    assert_eq!(vm.processes[0].registers[3 - 1], 914);
    run_for(&mut vm, 5);
    //ld 5 zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
#[ignore]
fn test() {
    todo!()
}
#[test]
#[ignore]
fn ldi_ind_reg() {
    todo!()
}
#[test]
#[ignore]
fn or_ind_reg() {
    todo!()
}

#[test]
fn xor_ind_ind() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_xor_ind_ind.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //xor 6
    run_for(&mut vm, 6);
    assert_eq!(vm.processes[0].registers[3 - 1], 247);
    run_for(&mut vm, 5);
    //ld 5 zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn ldi_reg_dir() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_ldi_reg_dir.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    // ld
    run_for(&mut vm, 5);
    //ldi 25
    run_for(&mut vm, 25);
    assert_eq!(vm.processes[0].registers[3 - 1], 133631); // ae4fffc
    // ld
    run_for(&mut vm, 5);
    //zjmp
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
#[ignore]
fn or_reg_ind() {
    todo!()
}
#[test]
#[ignore]
fn xor_ind_reg() {
    todo!()
}
#[test]
#[ignore]
fn ldi_reg_reg() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_ldi_reg_reg.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    // ld
    run_for(&mut vm, 5);
    //ldi 25
    run_for(&mut vm, 25);
    assert_eq!(vm.processes[0].registers[3 - 1], 521); // ae4fffc
    // ld
    run_for(&mut vm, 5);
    //zjmp
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
#[ignore]
fn or_reg_reg() {
    todo!()
}
#[test]
#[ignore]
fn xor_reg_ind() {
    todo!()
}
#[test]
#[ignore]
fn ld() {
    todo!()
}

#[test]
fn st_ind() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_st_ind.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena, vec![process]);

    vm.load_player(player);
    // live 10 ld 5 ld 5 add 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    // st
    // 01 ff ff ff ff 03 70 01 00 10 02 90 00 00 00 00 02 09 ff ef 00 ff ff ff ff 00 00 00 00 00 00 00
    // 01 FF FF FF FF 03 70 01 00 10 02 90 00 00 00 00 02 09 FF EF 00 FF FF FF FF 00 00 00 00 00 00 00
    for _ in 0..5 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    println!("{}", vm.arena);
    //let at_mem = vm.arena.read(vm.processes[0].instction_pc + 16, 4);
    let at_mem = vm.arena.read(21, 4);
    assert_eq!(vec![255, 255, 255, 255], at_mem);
    run_for(&mut vm, 5);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    //ld 5 zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
#[ignore]
fn xor_reg_reg() {
    todo!()
}

#[test]
fn add_ind_ind() {
    // This simulates what main() does
    let args = vec![
        "vm".into(),
        "playground/players_src/pierino_and_ind_ind.cor".into(),
    ];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10 ld 5 ld 5 add 10
    run_for(&mut vm, 10);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //and
    run_for(&mut vm, 6);
    assert_eq!(vm.processes[0].registers[3 - 1], 0x302);
    run_for(&mut vm, 5);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    //ld 5 zjmp 20
    run_for(&mut vm, 20);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

fn run_for(vm: &mut VirtualMachine, n: usize) {
    // zjmp 20
    for _ in 0..n {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
}
