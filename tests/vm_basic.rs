//////////////////use corewar::{Arena, Process, VirtualMachine, parse_arguments};
use vm::State;
use vm::*;
mod test_helpers;
use Instruction::*;
use test_helpers::*; // or specific imports
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
    let mut vm = build_vm("pierino_add");
    // live 10
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ld 5
    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[0].registers[2 - 1], 2);

    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[0].registers[3 - 1], 3);
    // add
    run_inst(&mut vm, Add);
    assert_eq!(vm.processes[0].registers[4 - 1], 5);
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[3 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn lld_dir_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_lld_dir_reg");
    // live 10
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);

    //lld 10
    run_inst(&mut vm, Lld);
    assert_eq!(vm.processes[0].registers[2 - 1], 1234); // ae4fffc

    //ld
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn sti_reg_dir_dir() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_sti_reg_dir_dir");
    // live 10
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    run_inst(&mut vm, Ld);
    //sti 25
    run_inst(&mut vm, Sti);
    //println!("{}", vm.arena);
    //01 ff ff ff ff 02 90 00 00 00 7b 02 ff ff ff ff
    //01 FF FF FF FF 02 90 00 00 00 7B 02 FF FF FF FF
    let at_mem = vm.arena.read(36, 4);
    assert_eq!(vec![255, 255, 255, 255], at_mem);
    run_inst(&mut vm, Ld);
    // zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn and_ind_ind() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_and_ind_ind");
    // live 10
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //and 6
    run_inst(&mut vm, And);
    assert_eq!(vm.processes[0].registers[3 - 1], 770);
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn lldi_dir_dir_reg() {
    let mut vm = build_vm("pierino_lldi_dir_dir_reg");
    // live 10
    println!("{}", vm.arena);
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //lldi 50
    run_inst(&mut vm, Lldi);
    assert_eq!(vm.processes[0].registers[3 - 1], 521); // 209ff //209FFED

    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn sti_reg_dir_reg() {
    let mut vm = build_vm("pierino_sti_reg_dir_reg");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    run_inst(&mut vm, Ld);
    //sti 25
    run_inst(&mut vm, Sti);
    //println!("{}", vm.arena);
    //01 ff ff ff ff 02 90 00 00 00 7b 02 ff ff ff ff
    //01 FF FF FF FF 02 90 00 00 00 7B 02 FF FF FF FF
    let at_mem = vm.arena.read(158, 4);
    assert_eq!(vec![255, 255, 255, 255], at_mem);
    run_inst(&mut vm, Ld);
    // zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn and_ind_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_and_ind_reg");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //and 6
    run_inst(&mut vm, And);
    assert_eq!(vm.processes[0].registers[3 - 1], -28672);
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn lldi_dir_reg_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_lldi_dir_reg_reg");
    /*-------------------------------------------------- */
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);

    //ld
    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[0].registers[2 - 1], 10);
    //lldi 50
    run_inst(&mut vm, Lldi);
    assert_eq!(vm.processes[0].registers[3 - 1], 133631); // 209ff //209FFED
    //ld
    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    // zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn sti_reg_ind_dir() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_sti_reg_ind_dir");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    run_inst(&mut vm, Ld);
    //sti 25
    run_inst(&mut vm, Sti);
    //println!("{}", vm.arena);
    //01 ff ff ff ff 02 90 00 00 00 7b 02 ff ff ff ff
    //01 FF FF FF FF 02 90 00 00 00 7B 02 FF FF FF FF
    let at_mem = vm.arena.read(12, 4);
    assert_eq!(vec![255, 255, 255, 255], at_mem);
    run_inst(&mut vm, Ld);
    // zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn and_reg_ind() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_and_reg_ind");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //and 6
    run_inst(&mut vm, And);
    assert_eq!(vm.processes[0].registers[3 - 1], 29697);
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn lldi_ind_dir_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_lldi_ind_dir_reg");
    // live 10
    println!("{}", vm.arena);
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //lldi 50
    run_inst(&mut vm, Lldi);
    assert_eq!(vm.processes[0].registers[3 - 1], 133631); // 209ff //209FFED

    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn sti_reg_ind_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_sti_reg_ind_reg");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    run_inst(&mut vm, Ld);
    //sti 25
    run_inst(&mut vm, Sti);
    //println!("{}", vm.arena);
    //01 ff ff ff ff 02 90 00 00 00 7b 02 ff ff ff ff
    //01 FF FF FF FF 02 90 00 00 00 7B 02 FF FF FF FF
    let at_mem = vm.arena.read(12, 4);
    assert_eq!(vec![255, 255, 255, 255], at_mem);
    run_inst(&mut vm, Ld);
    // zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn and_reg_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_and_reg_reg");
    // live 10
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    // ld
    run_inst(&mut vm, Ld);
    //and 6
    run_inst(&mut vm, And);
    assert_eq!(vm.processes[0].registers[3 - 1], 16);
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn lldi_ind_reg_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_lldi_ind_reg_reg");
    // live 10
    println!("{}", vm.arena);
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ld 5
    run_inst(&mut vm, Ld);
    //lldi 50
    run_inst(&mut vm, Lldi);
    assert_eq!(vm.processes[0].registers[3 - 1], 521); // 209ff //209FFED
    // ld
    run_inst(&mut vm, Ld);
    // zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn sti_reg_reg_dir() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_sti_reg_reg_dir");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    run_inst(&mut vm, Ld);
    //sti 25
    run_inst(&mut vm, Sti);
    //println!("{}", vm.arena);
    //01 ff ff ff ff 02 90 00 00 00 7b 02 ff ff ff ff
    //01 FF FF FF FF 02 90 00 00 00 7B 02 FF FF FF FF
    let at_mem = vm.arena.read(136, 4);
    assert_eq!(vec![255, 255, 255, 255], at_mem);
    run_inst(&mut vm, Ld);
    // zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn fork() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_fork");
    // fork 800
    run_inst(&mut vm, Fork);
    assert_eq!(vm.processes.len(), 2);
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);

    assert_eq!(vm.processes[1].live_status.executed, true);
    assert_eq!(vm.processes[1].live_status.player_id, -1);
    assert_eq!(vm.processes[1].live_status.nbr_live, 1);

    run_inst(&mut vm, Ld);
    // zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 3);

    assert_eq!(vm.processes[1].registers[2 - 1], 0);
    assert_eq!(vm.processes[1].pc.get(), 3);
}

#[test]
fn lld_ind_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_lld_ind_reg");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);

    //lld 10
    run_inst(&mut vm, Lld);
    assert_eq!(vm.processes[0].registers[2 - 1], -1); // ae4fffc

    //ld
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn sti_reg_reg_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_sti_reg_reg_reg");
    // live
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    run_inst(&mut vm, Ld);
    //sti 25
    run_inst(&mut vm, Sti);
    //println!("{}", vm.arena);
    //01 ff ff ff ff 02 90 00 00 00 7b 02 ff ff ff ff
    //01 FF FF FF FF 02 90 00 00 00 7B 02 FF FF FF FF
    let at_mem = vm.arena.read(135, 4);
    assert_eq!(vec![255, 255, 255, 255], at_mem);
    run_inst(&mut vm, Ld);
    // zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn ldi_dir_dir() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_ldi_dir_dir");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ldi 25
    run_inst(&mut vm, Ldi);
    assert_eq!(vm.processes[0].registers[3 - 1], 65537); // ae4fffc
    // ld
    run_inst(&mut vm, Ld);
    //zjmp
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn lldi_reg_dir_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_lldi_reg_dir_reg");
    // live 10
    println!("{}", vm.arena);
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ld
    run_inst(&mut vm, Ld);
    //lldi 50
    run_inst(&mut vm, Lldi);
    assert_eq!(vm.processes[0].registers[3 - 1], 241434880); // 209ff //209FFED
    //ld
    run_inst(&mut vm, Ld);
    // zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn st_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_st_reg");
    // live 10 ld 5 ld 5 add 10
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);

    // st
    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[0].registers[2 - 1], -1);

    // ld
    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn ldi_dir_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_ldi_dir_reg");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    // ld
    run_inst(&mut vm, Ld);
    //ldi 25
    run_inst(&mut vm, Ldi);
    assert_eq!(vm.processes[0].registers[3 - 1], 133631); // ae4fffc
    // ld
    run_inst(&mut vm, Ld);
    //zjmp
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn lldi_reg_reg_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_lldi_reg_reg_reg");
    // live 10
    println!("{}", vm.arena);
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ld
    run_inst(&mut vm, Ld);
    //lldi 50
    run_inst(&mut vm, Lldi);
    assert_eq!(vm.processes[0].registers[3 - 1], 521); // 209ff //209FFED
    //ld
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
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
    run_inst(&mut vm, Live);
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
    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[0].registers[3 - 1], 3);
    // sub
    run_inst(&mut vm, Sub);
    assert_eq!(vm.processes[0].registers[4 - 1], -1);
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn ldi_ind_dir() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_ldi_ind_dir");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ldi 25
    run_inst(&mut vm, Ldi);
    assert_eq!(vm.processes[0].registers[3 - 1], 182779900); // ae4fffc
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn or_ind_ind() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_or_ind_ind");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //or 6
    run_inst(&mut vm, Or);
    assert_eq!(vm.processes[0].registers[3 - 1], 914);
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn ldi_ind_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_ldi_ind_reg");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    // ld
    run_inst(&mut vm, Ld);
    //ldi 25
    run_inst(&mut vm, Ldi);
    assert_eq!(vm.processes[0].registers[3 - 1], 521); // ae4fffc
    // ld
    run_inst(&mut vm, Ld);
    //zjmp
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn or_ind_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_or_ind_reg");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //or
    run_inst(&mut vm, Or);
    assert_eq!(vm.processes[0].registers[3 - 1], -1);
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn xor_ind_ind() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_xor_ind_ind");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //xor 6
    run_inst(&mut vm, Xor);
    assert_eq!(vm.processes[0].registers[3 - 1], 247);
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn ldi_reg_dir() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_ldi_reg_dir");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    // ld
    run_inst(&mut vm, Ld);
    //ldi 25
    run_inst(&mut vm, Ldi);
    assert_eq!(vm.processes[0].registers[3 - 1], 133631); // ae4fffc
    // ld
    run_inst(&mut vm, Ld);
    //zjmp
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn or_reg_ind() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_or_reg_ind");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //or
    run_inst(&mut vm, Or);
    assert_eq!(vm.processes[0].registers[3 - 1], -1);
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn xor_ind_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_xor_ind_reg");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //xor 6
    run_inst(&mut vm, Xor);
    assert_eq!(vm.processes[0].registers[3 - 1], -2261);
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn ldi_reg_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_ldi_reg_reg");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    // ld
    run_inst(&mut vm, Ld);
    //ldi 25
    run_inst(&mut vm, Ldi);
    assert_eq!(vm.processes[0].registers[3 - 1], 521); // ae4fffc
    // ld
    run_inst(&mut vm, Ld);
    //zjmp
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn or_reg_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_or_reg_reg");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ld 5
    run_inst(&mut vm, Ld);
    //or 6
    run_inst(&mut vm, Or);
    assert_eq!(vm.processes[0].registers[3 - 1], 16);
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn xor_reg_ind() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_xor_reg_ind");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //xor 6
    run_inst(&mut vm, Xor);
    assert_eq!(vm.processes[0].registers[3 - 1], -2165);
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn ld() {
    // This simulates what main() does
    let args = vec!["vm".into(), "playground/players_src/pierino_ld.cor".into()];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ld 5
    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[0].registers[2 - 1], 517); // ae4fffc
    // ld
    run_inst(&mut vm, Ld);
    //zjmp
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn st_ind() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_st_ind");
    // live 10 ld 5 ld 5 add 10
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    // st
    // 01 ff ff ff ff 03 70 01 00 10 02 90 00 00 00 00 02 09 ff ef 00 ff ff ff ff 00 00 00 00 00 00 00
    // 01 FF FF FF FF 03 70 01 00 10 02 90 00 00 00 00 02 09 FF EF 00 FF FF FF FF 00 00 00 00 00 00 00
    run_inst(&mut vm, St);
    println!("{}", vm.arena);
    //let at_mem = vm.arena.read(vm.processes[0].instction_pc + 16, 4);
    let at_mem = vm.arena.read(21, 4);
    assert_eq!(vec![255, 255, 255, 255], at_mem);
    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
#[test]
fn xor_reg_reg() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_xor_reg_reg");
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    // ld
    run_inst(&mut vm, Ld);
    //xor 6
    run_inst(&mut vm, Xor);
    assert_eq!(vm.processes[0].registers[3 - 1], -17);
    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn add_ind_ind() {
    // This simulates what main() does
    let mut vm = build_vm("pierino_and_ind_ind");
    // live 10 ld 5 ld 5 add 10
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //and
    run_inst(&mut vm, And);
    assert_eq!(vm.processes[0].registers[3 - 1], 0x302);
    run_inst(&mut vm, Ld);
    //assert_eq!(vm.processes[0].registers[2 - 1], 0);
    does_reg(&vm, 2, 0);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn empty_player() {
    // This simulates what main() does
    let mut vm = build_vm("empty_player");
    // live 10 ld 5 ld 5 add 10
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].registers[0], -1); // for example
}

#[test]
fn pierino() {
    // This simulates what main() does
    let args = vec!["vm".into(), "playground/players_src/pierino.cor".into()];
    let player = parse_arguments(args).expect("parse failed");

    let arena = Arena::new();
    let process = Process::new(player.id, 0);

    println!("{player}");
    println!("{}", process);
    //println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process]);

    vm.load_player(player);
    // sti 10
    run_inst(&mut vm, Sti);

    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);

    // ld
    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[0].registers[2 - 1], 0);

    // zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].pc.get(), 7);
}

#[test]
fn pierino_test() {
    let mut vm = build_vm("pierino_test");

    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);

    run_inst(&mut vm, Ld);
    does_reg(&vm, 2, 2);
    assert_eq!(vm.processes[0].carry, false);

    run_inst(&mut vm, Ld);
    does_reg(&vm, 3, 3);
    assert_eq!(vm.processes[0].carry, false);

    run_inst(&mut vm, Add);
    does_reg(&vm, 4, 5);
    assert_eq!(vm.processes[0].carry, false);

    run_inst(&mut vm, Sub);
    does_reg(&vm, 5, 0);
    assert_eq!(vm.processes[0].carry, true);

    run_inst(&mut vm, And);
    does_reg(&vm, 6, 5);
    assert_eq!(vm.processes[0].carry, false);

    run_inst(&mut vm, Or);
    does_reg(&vm, 7, 0);
    assert_eq!(vm.processes[0].carry, true);

    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].pc.get(), 0)
}

#[test]
fn crab() {
    let mut vm = build_vm("crab");

    run_inst(&mut vm, St);
    does_reg(&vm, 8, -1);

    run_inst(&mut vm, Sti);
    run_inst(&mut vm, Sti);
    run_inst(&mut vm, Nop);
    run_inst(&mut vm, Nop);
    run_inst(&mut vm, Sti);
    run_inst(&mut vm, Sti);
    run_inst(&mut vm, Sti);
    run_inst(&mut vm, Sti);
}

#[test]
fn zjmp() {
    let mut vm = build_vm("zjmp");

    run_inst(&mut vm, Ld);
    does_reg(&vm, 2, 5);

    run_inst(&mut vm, Ld);
    does_reg(&vm, 3, 10);

    run_inst(&mut vm, And);
    does_reg(&vm, 4, 0);

    let current_pc = vm.processes[0].pc.get();
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[0].pc.get(), current_pc + 4);

    //run_inst(&mut vm, Ld); shoujd be skipped

    let current_pc = vm.processes[0].pc.get();
    println!("this is the headach current pc  {}", current_pc);
    run_inst(&mut vm, Zjmp); // jump back by 6
    println!("this is the pc after zjmp  {}", current_pc);
    assert_eq!(vm.processes[0].pc.get(), current_pc - 6);

    // run_inst(&mut vm, And);
    // does_reg(&vm, 5, 15);
    // run_inst(&mut vm, Sub);
    // does_reg(&vm, 6, 5);
}
