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
        match opcode {
            op0::rtrue => return Ok(Action::Return(1)),
            op0::rfalse => return Ok(Action::Return(0)),
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
                return Ok(Action::Return(1));
            }
            op0::nop => {}
            op0::save => {
                let ver = self.version();
                if ver >= Version::V5 {
                    return Err(ExecuteError::InvalidOpcode(opcode));
                }
                let saved = self.request_save();
                if ver < Version::V4 {
                    self.branch(saved, addr);
                } else {
                    self.store(saved as u16, addr);
                }
            }
            op0::restore => {
                let ver = self.version();
                if ver >= Version::V5 {
                    return Err(ExecuteError::InvalidOpcode(opcode));
                }
                let restored = self.request_restore();
                if ver < Version::V4 {
                    self.branch(restored, addr);
                } else {
                    self.store(restored as u16, addr);
                }
            }
            op0::restart => self.restart(),
            op0::ret_popped => return Ok(Action::Return(self.pop_stack())),
            op0::pop => {
                if self.version() < Version::V5 {
                    self.pop_stack();
                } else {
                    // also `catch`
                    let frame = self.stack_frame();
                    self.store(frame, addr);
                }
            }
            op0::quit => self.quit(),
            op0::new_line => self.print_newline(),
            op0::show_status => self.update_status_line(),
            op0::verify => {
                let checksum = self.calculate_checksum();
                let expected = self.word(ByteAddress::FILE_CHECKSUM);
                self.branch(checksum == expected, addr);
            }
            op0::extended => unreachable!(),
            op0::piracy => self.branch(true, addr),
            _ => return Err(ExecuteError::InvalidOpcode(opcode)),
        }
        Ok(Action::Continue)
    }
}
