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
fn test_1() {
    // This simulates what main() does
    let args = vec!["vm".into(), "playground/players_src/pierino_add.cor".into()];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // live 10
    for _ in 0..10 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
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
    // ld
    for _ in 0..5 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[3 - 1], 3);
    // add
    for _ in 0..10 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[4 - 1], 5);
    // ld
    for _ in 0..5 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    //ld 5 zjmp 20
    for _ in 0..20 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[3 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn test_2() {
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
    for _ in 0..10 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //and
    for _ in 0..6 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[3 - 1], 0x302);
    // ld
    for _ in 0..5 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    //ld 5 zjmp 20
    for _ in 0..20 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn test_3() {
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
    for _ in 0..10 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    // st
    for _ in 0..5 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    //let at_mem = vm.arena.read(vm.processes[0].instction_pc + 16, 4);
    let at_mem = vm.arena.read(21, 4);
    assert_eq!(vec![255, 255, 255, 255], at_mem);
    // ld
    for _ in 0..5 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    //ld 5 zjmp 20
    for _ in 0..20 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn test_4() {
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
    for _ in 0..10 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
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
    // ld
    for _ in 0..5 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[3 - 1], 3);
    // sub
    for _ in 0..10 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[4 - 1], -1);
    // ld
    for _ in 0..5 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    //ld 5 zjmp 20
    for _ in 0..20 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn test_and() {
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
    for _ in 0..10 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //and 6
    for _ in 0..6 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[3 - 1], 770);
    // ld
    for _ in 0..5 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    //ld 5 zjmp 20
    for _ in 0..20 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn test_or() {
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
    for _ in 0..10 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //or 6
    for _ in 0..6 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[3 - 1], 914);
    // ld
    for _ in 0..5 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    //ld 5 zjmp 20
    for _ in 0..20 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn test_xor() {
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
    for _ in 0..10 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //xor 6
    for _ in 0..6 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[3 - 1], 247);
    // ld
    for _ in 0..5 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    //ld 5 zjmp 20
    for _ in 0..20 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn test_ldi() {
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
    for _ in 0..10 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ldi 25
    for _ in 0..25 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[3 - 1], 182779900); // ae4fffc
    // ld
    for _ in 0..5 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    //ld 5 zjmp 20
    for _ in 0..20 {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena);
            }
        }
        vm.cycle();
    }
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
