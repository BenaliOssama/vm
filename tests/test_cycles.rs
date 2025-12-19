use vm::State;
use vm::helper::*;
use vm::*;
mod test_helpers;
use Instruction::*;
use test_helpers::*;

#[test]
#[ignore]
fn add_cycles() {
    let mut vm = build_vm("pierino_add");
    // live 10
    for _ in 0..1537 {
        vm.cycle();
    }
    for _ in 0..4000 {
        vm.cycle();
    }
    assert_eq!(vm.cycles_to_die, 1486);
    //assert_eq!(vm.cycle_count, 1537);
    // assert_eq!(vm.cycles_to_die, 1487);
}
