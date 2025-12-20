use vm::process;

mod common;
use common::*;

#[test]
fn cycles_add() {
    let mut vm = build_vm("pierino_add");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (23990, 1036),
        (35397, 986),
        (46254, 936),
        (56561, 886),
        (66318, 836),
        (75525, 786),
        (84182, 736),
        (92289, 686),
        (99846, 636),
        (106853, 586),
        (113310, 536),
        (119217, 486),
        (124574, 436),
        (129381, 386),
        (133638, 336),
        (137345, 286),
        (140502, 236),
        (143109, 186),
        (145166, 136),
        (146673, 86),
        (147630, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            for process in &mut vm.processes {
                if process.state() == process::State::NoInstruction {
                    process.fetch_decode(&mut vm.arena, vm.cycle_count);
                }
            }
            vm.cycle_count += 1;
            vm.cycle();
            vm.cycle_logic();
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
}

#[test]
#[ignore]
fn cycles_crab() {
    todo!()
}
#[test]
#[ignore]
fn cycles_ldi_ind_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_lldi_reg_reg_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_sti_reg_reg_dir() {
    todo!()
}
#[test]
#[ignore]
fn cycles_empty_player() {
    todo!()
}
#[test]
#[ignore]
fn cycles_ldi_reg_dir() {
    todo!()
}
#[test]
#[ignore]
fn cycles_or_ind_ind() {
    todo!()
}
#[test]
#[ignore]
fn cycles_sti_reg_reg_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_ldi_reg_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_or_ind_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_st_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_and_ind_ind() {
    todo!()
}
#[test]
#[ignore]
fn cycles_ld() {
    todo!()
}
#[test]
#[ignore]
fn cycles_or_reg_ind() {
    todo!()
}
#[test]
#[ignore]
fn cycles_sub() {
    todo!()
}
#[test]
#[ignore]
fn cycles_and_ind_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_lld_dir_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_or_reg_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_test() {
    todo!()
}
#[test]
#[ignore]
fn cycles_and_reg_ind() {
    todo!()
}
#[test]
#[ignore]
fn cycles_lldi_dir_dir_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_pierino() {
    todo!()
}
#[test]
#[ignore]
fn cycles_xor_ind_ind() {
    todo!()
}
#[test]
#[ignore]
fn cycles_and_reg_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_lldi_dir_reg_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_st_ind() {
    todo!()
}
#[test]
#[ignore]
fn cycles_xor_ind_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_fork() {
    todo!()
}
#[test]
#[ignore]
fn cycles_lldi_ind_dir_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_sti_reg_dir_dir() {
    todo!()
}
#[test]
#[ignore]
fn cycles_xor_reg_ind() {
    todo!()
}
#[test]
#[ignore]
fn cycles_ldi_dir_dir() {
    todo!()
}
#[test]
#[ignore]
fn cycles_lldi_ind_reg_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_sti_reg_dir_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_xor_reg_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_ldi_dir_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_lld_ind_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_sti_reg_ind_dir() {
    todo!()
}
#[test]
#[ignore]
fn cycles_zjmp() {
    todo!()
}
#[test]
#[ignore]
fn cycles_ldi_ind_dir() {
    todo!()
}
#[test]
#[ignore]
fn cycles_lldi_reg_dir_reg() {
    todo!()
}
#[test]
#[ignore]
fn cycles_sti_reg_ind_reg() {
    todo!()
}
