use super::*;
use crate::*;
use opcodes::op0;

impl ZMachine {
    crate fn execute_op0(
        &mut self,
        addr: &mut ByteAddress,
        desc: OperandsDesc,
        opcode: u8,
    ) -> ExecuteResult {
        fn branch(z: &mut ZMachine, success: bool, addr: &mut ByteAddress) {
            let top = z[*addr];
            *addr += 1;
            let branch = if top & 0b1_0000000 == 0 {
                !success
            } else {
                success
            };
            let addr = if top & 0b0_1_000000 == 0b0_1_000000 {
                *addr + (top & 0b00_111111) as usize
            } else {
                let bottom = z[*addr];
                *addr += 1;
                let signed = i16::from_be_bytes(((top as u16) << 8 | bottom as u16).to_be_bytes());
                if signed >= 0 {
                    *addr + (signed as usize)
                } else {
                    *addr + ((-signed) as usize)
                }
            };
            z.jump(addr - 2);
        }
        fn store(z: &mut ZMachine, value: u16, addr: &mut ByteAddress) {
            let var = z[*addr];
            *addr += 1;
            z.set_local_var(var, value);
        }
        match opcode {
            op0::rtrue => return Ok(Some(1)),
            op0::rfalse => return Ok(Some(0)),
            op0::print => {
                let (str, bytes) = self.read_zstring(*addr);
                self.print(&str);
                *addr += bytes;
            }
            op0::print_ret => {
                let (str, bytes) = self.read_zstring(*addr);
                self.print(&str);
                self.print_newline();
                *addr += bytes;
                return Ok(Some(1));
            }
            op0::nop => {}
            op0::save => {
                let ver = self.version();
                if ver >= Version::V5 {
                    return Err(ExecuteError::InvalidOpcode(opcode));
                }
                let saved = self.request_save();
                if ver < Version::V4 {
                    branch(self, saved, addr);
                } else {
                    store(self, saved as u16, addr);
                }
            }
            op0::restore => {
                let ver = self.version();
                if ver >= Version::V5 {
                    return Err(ExecuteError::InvalidOpcode(opcode));
                }
                let restored = self.request_restore();
                if ver < Version::V4 {
                    branch(self, restored, addr);
                } else {
                    store(self, restored as u16, addr);
                }
            }
            op0::restart => self.restart(),
            op0::ret_popped => return Ok(Some(self.pop_stack())),
            op0::pop => {
                if self.version() < Version::V5 {
                    self.pop_stack();
                } else {
                    // also `catch`
                    let frame = self.stack_frame();
                    store(self, frame, addr);
                }
            }
            op0::quit => self.quit(),
            op0::new_line => self.print_newline(),
            op0::show_status => self.update_status_line(),
            op0::verify => {
                let checksum = self.calculate_checksum();
                let expected = self.word(ByteAddress::FILE_CHECKSUM);
                branch(self, checksum == expected, addr);
            }
            op0::extended => {}
            _ => return Err(ExecuteError::InvalidOpcode(opcode)),
        }
        Ok(None)
    }
}
