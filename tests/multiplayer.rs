mod common;
use common::Instruction::*;
use common::*;

#[test]
fn two() {
    let mut vm = build_vm_more(vec!["pierino_add", "pierino_add"]);
    // live 10
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[1].live_status.executed, true);
    assert_eq!(vm.processes[1].live_status.player_id, -1);
    assert_eq!(vm.processes[1].live_status.nbr_live, 1);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ld 5
    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[1].registers[2 - 1], 2);
    assert_eq!(vm.processes[0].registers[2 - 1], 2);

    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[1].registers[3 - 1], 3);
    assert_eq!(vm.processes[0].registers[3 - 1], 3);
    // add
    run_inst(&mut vm, Add);
    assert_eq!(vm.processes[1].registers[4 - 1], 5);
    assert_eq!(vm.processes[0].registers[4 - 1], 5);

    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[1].registers[3 - 1], 0);
    assert_eq!(vm.processes[1].pc.get(), 2048);
    assert_eq!(vm.processes[0].registers[3 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn three() {
    let mut vm = build_vm_more(vec!["pierino_add", "pierino_add", "pierino_add"]);
    // live 10
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[2].live_status.executed, true);
    assert_eq!(vm.processes[2].live_status.player_id, -1);
    assert_eq!(vm.processes[2].live_status.nbr_live, 1);
    assert_eq!(vm.processes[1].live_status.executed, true);
    assert_eq!(vm.processes[1].live_status.player_id, -1);
    assert_eq!(vm.processes[1].live_status.nbr_live, 1);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ld 5
    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[2].registers[2 - 1], 2);
    assert_eq!(vm.processes[1].registers[2 - 1], 2);
    assert_eq!(vm.processes[0].registers[2 - 1], 2);

    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[2].registers[3 - 1], 3);
    assert_eq!(vm.processes[1].registers[3 - 1], 3);
    assert_eq!(vm.processes[0].registers[3 - 1], 3);
    // add
    run_inst(&mut vm, Add);
    assert_eq!(vm.processes[2].registers[4 - 1], 5);
    assert_eq!(vm.processes[1].registers[4 - 1], 5);
    assert_eq!(vm.processes[0].registers[4 - 1], 5);

    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[2].registers[3 - 1], 0);
    assert_eq!(vm.processes[2].pc.get(), 2730);
    assert_eq!(vm.processes[1].registers[3 - 1], 0);
    assert_eq!(vm.processes[1].pc.get(), 1365);
    assert_eq!(vm.processes[0].registers[3 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn four() {
    let mut vm = build_vm_more(vec![
        "pierino_add",
        "pierino_add",
        "pierino_add",
        "pierino_add",
    ]);
    // live 10
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[3].live_status.executed, true);
    assert_eq!(vm.processes[3].live_status.player_id, -1);
    assert_eq!(vm.processes[3].live_status.nbr_live, 1);
    assert_eq!(vm.processes[2].live_status.executed, true);
    assert_eq!(vm.processes[2].live_status.player_id, -1);
    assert_eq!(vm.processes[2].live_status.nbr_live, 1);
    assert_eq!(vm.processes[1].live_status.executed, true);
    assert_eq!(vm.processes[1].live_status.player_id, -1);
    assert_eq!(vm.processes[1].live_status.nbr_live, 1);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ld 5
    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[3].registers[2 - 1], 2);
    assert_eq!(vm.processes[2].registers[2 - 1], 2);
    assert_eq!(vm.processes[1].registers[2 - 1], 2);
    assert_eq!(vm.processes[0].registers[2 - 1], 2);

    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[3].registers[3 - 1], 3);
    assert_eq!(vm.processes[2].registers[3 - 1], 3);
    assert_eq!(vm.processes[1].registers[3 - 1], 3);
    assert_eq!(vm.processes[0].registers[3 - 1], 3);
    // add
    run_inst(&mut vm, Add);
    assert_eq!(vm.processes[3].registers[4 - 1], 5);
    assert_eq!(vm.processes[2].registers[4 - 1], 5);
    assert_eq!(vm.processes[1].registers[4 - 1], 5);
    assert_eq!(vm.processes[0].registers[4 - 1], 5);

    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[3].registers[3 - 1], 0);
    assert_eq!(vm.processes[3].pc.get(), 1024 * 3);
    assert_eq!(vm.processes[2].registers[3 - 1], 0);
    assert_eq!(vm.processes[2].pc.get(), 2048);
    assert_eq!(vm.processes[1].registers[3 - 1], 0);
    assert_eq!(vm.processes[1].pc.get(), 1024);
    assert_eq!(vm.processes[0].registers[3 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn add_sub_ld() {
    let mut vm = build_vm_more(vec!["add", "sub", "ld", "xor"]);
    // live 10
    run_inst(&mut vm, Live);

    assert_eq!(vm.processes[3].live_status.executed, true);
    assert_eq!(vm.processes[3].live_status.player_id, -4);
    assert_eq!(vm.processes[3].live_status.nbr_live, 1);

    assert_eq!(vm.processes[2].live_status.executed, true);
    assert_eq!(vm.processes[2].live_status.player_id, -3);
    assert_eq!(vm.processes[2].live_status.nbr_live, 1);

    assert_eq!(vm.processes[1].live_status.executed, true);
    assert_eq!(vm.processes[1].live_status.player_id, -2);
    assert_eq!(vm.processes[1].live_status.nbr_live, 1);

    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);

    // live 10
}

#[test]
fn add_sub_ld_xor() {
    let mut vm = build_vm_more(vec!["add", "sub", "ld", "xor"]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 28729);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 3);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino ld"
    );
}

#[test]
fn pierino_pierino_lldi_ind_dir_reg_pierino_ldi_dir_dir() {
    let mut vm = build_vm_more(vec![
        "pierino",
        "pierino_lldi_ind_dir_reg",
        "pierino_ldi_dir_dir",
    ]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 37488);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}

#[test]
fn pierino_lld_dir_reg_pierino_ldi_ind_dir_crab_pierino_and_reg_reg() {
    let mut vm = build_vm_more(vec![
        "pierino_lld_dir_reg",
        "pierino_ldi_ind_dir",
        "crab",
        "pierino_and_reg_reg",
    ]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 33895);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}

#[test]
fn add_pierino_st_reg_pierino_st_ind() {
    let mut vm = build_vm_more(vec!["add", "pierino_st_reg", "pierino_st_ind"]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 31849);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "add"
    );
}

#[test]
fn pierino_or_reg_reg_pierino_and_ind_ind_pierino_lldi_reg_reg_reg() {
    let mut vm = build_vm_more(vec![
        "pierino_or_reg_reg",
        "pierino_and_ind_ind",
        "pierino_lldi_reg_reg_reg",
    ]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 34080);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}

#[test]
#[ignore]
fn pierino_ldi_ind_dir_empty_player_add_pierino_or_reg_reg() {
    let mut vm = build_vm_more(vec![
        "pierino_ldi_ind_dir",
        "empty_player",
        "add",
        "pierino_or_reg_reg",
    ]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 30951);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}

#[test]
fn pierino_sub_pierino_sti_reg_reg_reg_pierino_lld_ind_reg_pierino_xor_reg_reg() {
    let mut vm = build_vm_more(vec![
        "pierino_sub",
        "pierino_sti_reg_reg_reg",
        "pierino_lld_ind_reg",
        "pierino_xor_reg_reg",
    ]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 30914);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}

#[test]
fn pierino_and_ind_reg_pierino_st_ind_pierino_lld_ind_reg() {
    let mut vm = build_vm_more(vec![
        "pierino_and_ind_reg",
        "pierino_st_ind",
        "pierino_lld_ind_reg",
    ]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 31535);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}

#[test]
fn sub_pierino_ldi_dir_reg_pierino_sti_reg_dir_reg_pierino_sti_reg_dir_dir() {
    let mut vm = build_vm_more(vec![
        "sub",
        "pierino_ldi_dir_reg",
        "pierino_sti_reg_dir_reg",
        "pierino_sti_reg_dir_dir",
    ]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 31238);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 2);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}

#[test]
fn pierino_xor_reg_ind_pierino_xor_ind_ind_pierino_st_ind() {
    let mut vm = build_vm_more(vec![
        "pierino_xor_reg_ind",
        "pierino_xor_ind_ind",
        "pierino_st_ind",
    ]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 31210);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}

#[test]
#[ignore]
fn pierino_pierino_or_reg_ind_pierino_lldi_ind_dir_reg_pierino_ldi_reg_dir() {
    let mut vm = build_vm_more(vec![
        "pierino",
        "pierino_or_reg_ind",
        "pierino_lldi_ind_dir_reg",
        "pierino_ldi_reg_dir",
    ]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 31248);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}

#[test]
fn pierino_and_ind_ind_pierino_ldi_dir_dir_pierino_xor_ind_reg_pierino_or_reg_reg() {
    let mut vm = build_vm_more(vec![
        "pierino_and_ind_ind",
        "pierino_ldi_dir_dir",
        "pierino_xor_ind_reg",
        "pierino_or_reg_reg",
    ]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 29077);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}

#[test]
fn pierino_lld_ind_reg_pierino_or_reg_reg_sub() {
    let mut vm = build_vm_more(vec!["pierino_lld_ind_reg", "pierino_or_reg_reg", "sub"]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 33895);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}

#[test]
#[ignore]
fn pierino_ldi_reg_reg_pierino_sti_reg_ind_dir_pierino_lldi_reg_dir_reg() {
    let mut vm = build_vm_more(vec![
        "pierino_lldi_reg_dir_reg",
        "pierino_ldi_reg_reg",
        "pierino_sti_reg_ind_dir",
    ]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    println!("winner: {:?}", vm.winners);
    assert_eq!(vm.cycle_count, 50764);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}

#[test]
#[ignore = "should be applied after basic vm pass"]
fn dwarf_ameba() {
    let mut vm = build_vm_more(vec!["dwarf", "ameba"]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    println!("winner: {:?}", vm.winners);
    assert_eq!(vm.cycle_count, 50764);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "dwarf"
    );
}

#[test]
#[ignore = "should be applied after basic vm pass"]
fn ameba_dwarf() {
    let mut vm = build_vm_more(vec!["ameba", "dwarf"]);
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    println!("winner: {:?}", vm.winners);
    assert_eq!(vm.cycle_count, 50764);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
