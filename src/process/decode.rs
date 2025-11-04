use super::Process;
use crate::ParamType;
use crate::arena::*;
use crate::config::MEM_SIZE;
use crate::instructions::*;

impl Process {
    pub fn decode(&mut self, opcode: u8, arena: &mut Arena) -> Option<Instruction> {
        let inst_index = (opcode - 1) as usize;
        let inst_info = INSTRUCTION_TABLE[inst_index]; // instructions table is 1-indexed

        match opcode {
            // -------------------------------------------------------------------------
            // live %<direct>
            // -------------------------------------------------------------------------
            1 => {
                // live has no pcode, always 4-byte direct
                let bytes = arena.read(self.pc.get(), 4);
                self.pc.add(4);
                let value = bytes_to_i32(&bytes);

                self.remaining_cycles = inst_info.nb_cycles.saturating_sub(2);
                println!("{}: {}", vm::cyan("LIVE param"), value);

                Some(Instruction::new(opcode, vec![Parameter::Direct(value)]))
            }

            // -------------------------------------------------------------------------
            // ld <direct|indirect>, <register>
            // -------------------------------------------------------------------------
            2 => {
                // read and decode pcode
                let pcode = arena.read(self.pc.get(), 1)[0];
                self.pc.inc();

                let type_params = decode_pcode(pcode, inst_info.nb_params);

                // validate params
                let first_ok = matches!(
                    type_params.get(0),
                    Some(ParamType::Direct | ParamType::Indirect)
                );
                let second_ok = matches!(type_params.get(1), Some(ParamType::Register));
                if !first_ok || !second_ok {
                    eprintln!(
                        "Invalid parameter types for ld: {:?} {:?}",
                        type_params.get(0),
                        type_params.get(1)
                    );
                    return None;
                }

                // decode parameters
                let mut params = Vec::new();
                for param_type in type_params.iter() {
                    let param = match param_type {
                        ParamType::Direct => {
                            let size = if inst_info.has_idx { 2 } else { 4 };
                            let bytes = arena.read(self.pc.get(), size);
                            self.pc.add(size);
                            Parameter::Direct(bytes_to_i32(&bytes))
                        }
                        ParamType::Indirect => {
                            let bytes = arena.read(self.pc.get(), 2);
                            self.pc.add(2);
                            let offset = bytes_to_i16(&bytes);
                            let addr = wrap_address(self.pc.get(), offset);
                            let value = bytes_to_i32(&arena.read(addr, 4));
                            Parameter::Indirect(value)
                        }
                        ParamType::Register => {
                            let reg = arena.read(self.pc.get(), 1)[0] as usize;
                            self.pc.inc();
                            Parameter::Register(reg)
                        }
                        _ => Parameter::None,
                    };
                    params.push(param);
                }

                Some(Instruction::new(opcode, params))
            }

            // -------------------------------------------------------------------------
            _ => {
                eprintln!("Unknown opcode {}", opcode);
                None
            }
        }
    }
}

fn decode_pcode(pcode: u8, num_args: usize) -> [ParamType; 3] {
    let mut result = [ParamType::None; 3];

    for i in 0..num_args {
        let shift = 6 - (i * 2);
        let bits = (pcode >> shift) & 0b11;
        result[i] = match bits {
            0b01 => ParamType::Register,
            0b10 => ParamType::Direct,
            0b11 => ParamType::Indirect,
            _ => ParamType::None, // 0b00 means unused/invalid
        };
    }

    result
}
fn bytes_to_i32(bytes: &[u8]) -> i32 {
    let mut arr = [0u8; 4]; // 4 bytes for i32
    let len = bytes.len();
    // copy bytes to the end of the array (big-endian)
    arr[4 - len..].copy_from_slice(bytes);
    i32::from_be_bytes(arr)
}

fn bytes_to_i16(bytes: &[u8]) -> i16 {
    let mut arr = [0u8; 2]; // 2 bytes for i16
    let len = bytes.len();
    arr[2 - len..].copy_from_slice(bytes);
    i16::from_be_bytes(arr)
}

fn wrap_address(pc: usize, offset: i16) -> usize {
    let mut addr = (pc as isize + offset as isize) % MEM_SIZE as isize;
    if addr < 0 {
        addr += MEM_SIZE as isize;
    }
    addr as usize
}
