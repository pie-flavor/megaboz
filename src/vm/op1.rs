use super::*;
use crate::*;
use arrayvec::ArrayVec;
use opcodes::op1;

impl ZMachine {
    crate fn execute_op1(
        &mut self,
        addr: &mut ByteAddress,
        desc: OperandsDesc,
        opcode: u8,
    ) -> ExecuteResult {
        let operand_type_byte = self[*addr];
        *addr += 1;
        let operand_type = operand_type_byte >> 6;
        let operand_lit = match operand_type {
            0b00 => {
                let operand_word = self.word(*addr);
                *addr += 2;
                Operand::LargeConstant(operand_word)
            }
            0b01 => {
                let operand_byte = self[*addr];
                *addr += 1;
                Operand::SmallConstant(operand_byte)
            }
            0b10 => {
                let operand_byte = self[*addr];
                *addr += 1;
                Operand::Variable(operand_byte)
            }
            0b11 => Operand::Omitted,
            _ => unreachable!(),
        };
        let operand = operand_lit
            .resolve(self)
            .ok_or(ExecuteError::InvalidInstructionFormat(addr.0))?;
        match opcode {
            op1::jz => self.branch(operand == 0, addr),
            op1::get_sibling => {
                let obj = self.object_unchecked(operand as usize);
                if let Some(sibling) = obj.sibling_id() {
                    self.store(sibling as u16, addr);
                    self.branch(true, addr)
                } else {
                    self.store(0, addr);
                    self.branch(false, addr);
                }
            }
            op1::get_child => {
                let obj = self.object_unchecked(operand as usize);
                if let Some(child) = obj.child_id() {
                    self.store(child as u16, addr);
                    self.branch(true, addr);
                } else {
                    self.store(0, addr);
                    self.branch(false, addr);
                }
            }
            op1::get_parent => {
                let obj = self.object_unchecked(operand as usize);
                if let Some(parent) = obj.parent_id() {
                    self.store(parent as u16, addr);
                } else {
                    self.store(0, addr);
                }
            }
            op1::get_prop_len => {
                if operand == 0 {
                    self.store(0, addr);
                } else {
                    let prop_addr = ByteAddress::from(operand);
                    let sz_byte = self[prop_addr - 1];
                    let sz = if self.version() > Version::V3 {
                        if sz_byte & 0b1_0000000 == 0b1_0000000 {
                            sz_byte & 0b000_11111
                        } else if sz_byte & 0b0_1_000000 == 0b0_1_000000 {
                            2
                        } else {
                            1
                        }
                    } else {
                        (sz_byte >> 5) + 1
                    };
                    self.store(sz_byte as u16, addr);
                }
            }
            op1::inc => {
                let operand = operand as i16;
                let new_value = operand.wrapping_add(1);
                let varnum = match operand_lit {
                    Operand::Variable(varnum) => varnum,
                    _ => return Err(ExecuteError::InvalidInstructionFormat(addr.0)),
                };
                self.set_variable(varnum, new_value as u16);
            }
            op1::dec => {
                let operand = operand as i16;
                let new_value = operand.wrapping_sub(1);
                let varnum = match operand_lit {
                    Operand::Variable(varnum) => varnum,
                    _ => return Err(ExecuteError::InvalidInstructionFormat(addr.0)),
                };
                self.set_variable(varnum, new_value as u16);
            }
            op1::print_addr => {
                let string = self.read_zstring(operand.into()).0;
                self.print(&string);
            }
            op1::call_1s if self.version() >= Version::V4 => {
                let var = self.read_store(addr);
                return Ok(Action::Call {
                    addr: operand.into(),
                    retvar: Some(var),
                    args: ArrayVec::new(),
                });
            }
            op1::remove_obj => {
                let mut obj = self.object_unchecked(operand as _);
                obj.detach();
            }
            op1::print_obj => {
                let obj = self.object_unchecked(operand as _);
                let name = obj.read_name();
                self.print(&name);
            }
            op1::ret => return Ok(Action::Return(operand)),
            op1::jump => {
                let offset = operand as i16 as isize - 2;
                let addr = if offset < 0 {
                    *addr - ((-offset) as usize)
                } else {
                    *addr + offset as usize
                };
                self.jump(addr);
            }
            op1::print_paddr => {
                let high = self.resolve_packed_address(operand as usize, false);
                let string = self.read_zstring(high).0;
                self.print(&string);
            }
            op1::load => {
                let var = self.variable(operand as _);
                self.store(var, addr);
            }
            op1::not => {
                if self.version() <= Version::V4 {
                    self.store(!operand, addr);
                } else {
                    // also `call_1n`
                    return Ok(Action::Call {
                        addr: operand.into(),
                        retvar: None,
                        args: ArrayVec::new(),
                    });
                }
            }
            _ => return Err(ExecuteError::InvalidOpcode(opcode)),
        }
        Ok(Action::Continue)
    }
}
