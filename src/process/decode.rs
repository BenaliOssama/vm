use super::Process;
use crate::ParamType;
use crate::arena;
use crate::arena::*;
use crate::config::MEM_SIZE;
use crate::helper;
use crate::instructions;
use crate::instructions::*;

impl Process {
    pub fn decode(&mut self, opcode: u8, arena: &mut Arena) -> Option<Instruction> {
        let inst_index = (opcode - 1) as usize;
        let inst_info = INSTRUCTION_TABLE[inst_index]; // instructions table is 1-indexed
        self.remaining_cycles = inst_info.nb_cycles.saturating_sub(2);

        match opcode {
            // -------------------------------------------------------------------------
            // live %<direct>
            // -------------------------------------------------------------------------
            1 => {
                // live has no pcode, always 4-byte direct
                let bytes = arena.read(self.pc.get(), 4);
                self.pc.add(4);
                let value = helper::bytes_to_i32(&bytes);

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

                let params = self.build_params(type_params, inst_info, arena);
                // decode parameters
                Some(Instruction::new(opcode, params))
            }

            3 => {
                // return the add instruction
                // read and decode pcode
                let pcode = arena.read(self.pc.get(), 1)[0];
                self.pc.inc();

                let type_params = decode_pcode(pcode, inst_info.nb_params);
                // validate params
                // validate params
                let first_ok = matches!(type_params.get(0), Some(ParamType::Register));

                let second_ok = matches!(
                    type_params.get(1),
                    Some(ParamType::Indirect) | Some(ParamType::Register)
                );

                if !first_ok || !second_ok {
                    eprintln!(
                        "Invalid parameter types for st: {:?} {:?}",
                        type_params.get(0),
                        type_params.get(1)
                    );
                    return None;
                }

                let params = self.build_params(type_params, inst_info, arena);

                // decode parameters
                println!("ls parms $$$ {:?}", params);
                Some(Instruction::new(opcode, params))
            }
            4 => {
                // return the add instruction
                // read and decode pcode
                let pcode = arena.read(self.pc.get(), 1)[0];
                self.pc.inc();

                let type_params = decode_pcode(pcode, inst_info.nb_params);
                // validate params
                // validate params
                let first_ok = matches!(type_params.get(0), Some(ParamType::Register));
                let second_ok = matches!(type_params.get(1), Some(ParamType::Register));
                let third_ok = matches!(type_params.get(2), Some(ParamType::Register));

                if !first_ok || !second_ok || !third_ok {
                    eprintln!(
                        "Invalid parameter types for st: {:?} {:?} {:?}",
                        type_params.get(0),
                        type_params.get(1),
                        type_params.get(2),
                    );
                    return None;
                }

                let params = self.build_params(type_params, inst_info, arena);

                // decode parameters
                println!("ls parms $$$ {:?}", params);
                Some(Instruction::new(opcode, params))
            }
            // -------------------------------------------------------------------------
            _ => {
                eprintln!("Unknown opcode {}", opcode);
                None
            }
        }
    }

    fn build_params(
        &mut self,
        type_params: [ParamType; 3],
        inst_info: InstructionInfo,
        arena: &mut Arena,
    ) -> Vec<instructions::Parameter> {
        let mut params = Vec::new();
        for param_type in type_params.iter() {
            let param = match param_type {
                ParamType::Direct => {
                    let size = if inst_info.has_idx { 2 } else { 4 };
                    let bytes = arena.read(self.pc.get(), size);
                    self.pc.add(size);
                    Parameter::Direct(helper::bytes_to_i32(&bytes))
                }
                ParamType::Indirect => {
                    let bytes = arena.read(self.pc.get(), 2);
                    self.pc.add(2);
                    let offset = helper::bytes_to_i16(&bytes);
                    Parameter::Indirect(offset as i32)
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
        return params.clone();
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
