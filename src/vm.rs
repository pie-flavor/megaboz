#![allow(unused)]

use crate::*;
use arrayvec::ArrayVec;
use failure::Fail;

mod op0;
pub mod opcodes;
pub use self::op0::*;
mod op1;
pub use self::op1::*;
mod op2;
pub use self::op2::*;
mod var;
pub use self::var::*;
mod ext;
pub use self::ext::*;

impl ZMachine {
    fn execute(&mut self, addr: &mut ByteAddress) -> ExecuteResult {
        let opcode_byte = self[*addr];
        let (form, operands_desc, opcode) = match opcode_byte & 0b_11_000000 {
            0b_11_000000 => (
                OpcodeForm::Variable,
                if opcode_byte & 0b00_1_00000 == 0b00_1_00000 {
                    OperandsDesc::Var
                } else {
                    OperandsDesc::Op2
                },
                opcode_byte & 0b000_11111,
            ),
            0b_10_000000 => (
                OpcodeForm::Short,
                if opcode_byte & 0b00_11_0000 == 0b00_11_0000 {
                    OperandsDesc::Op0
                } else {
                    OperandsDesc::Op1
                },
                opcode_byte & 0b0000_1111,
            ),
            _ => {
                if opcode_byte == 190 && self.version() >= Version::V5 {
                    *addr += 1;
                    (OpcodeForm::Extended, OperandsDesc::Var, self[*addr])
                } else {
                    (
                        OpcodeForm::Long,
                        OperandsDesc::Op2,
                        opcode_byte & 0b000_11111,
                    )
                }
            }
        };
        *addr += 1;
        match operands_desc {
            OperandsDesc::Op0 => self.execute_op0(addr, operands_desc, opcode),
            _ => unimplemented!(),
        }
    }
    fn branch(&mut self, success: bool, addr: &mut ByteAddress) {
        let top = self[*addr];
        *addr += 1;
        let branch = if top & 0b1_0000000 == 0 {
            !success
        } else {
            success
        };
        let addr = if top & 0b0_1_000000 == 0b0_1_000000 {
            *addr + (top & 0b00_111111) as usize
        } else {
            let bottom = self[*addr];
            *addr += 1;
            let signed = (((top & 0b00_111111) as u16) << 8 | bottom as u16) as i16;
            if signed >= 0 {
                *addr + (signed as usize)
            } else {
                *addr - ((-signed) as usize)
            }
        };
        if branch {
            self.jump(addr - 2);
        }
    }
    fn store(&mut self, value: u16, addr: &mut ByteAddress) {
        let var = self[*addr];
        *addr += 1;
        self.set_variable(var, value);
    }
    fn read_store(&self, addr: &mut ByteAddress) -> u8 {
        let var = self[*addr];
        *addr += 1;
        var
    }
    /// Performs an execution jump to a particular address.
    pub fn jump(&mut self, addr: ByteAddress) {
        unimplemented!();
    }
    /// Prints a string to the screen.
    pub fn print(&mut self, string: &str) {
        unimplemented!();
    }
    /// Prints a newline to the screen.
    pub fn print_newline(&mut self) {
        unimplemented!();
    }
    /// Asks the user whether they want to save the game. Returns whether or not they did.
    pub fn request_save(&mut self) -> bool {
        unimplemented!()
    }
    /// Asks the user whether they want to restore the game from a save. Returns whether or not they
    /// did.
    pub fn request_restore(&mut self) -> bool {
        unimplemented!()
    }
    /// Returns the value of a global variable or local variable in the current routine.
    pub fn variable(&self, var: u8) -> u16 {
        unimplemented!()
    }
    /// Sets a global variable or local variable in the current routine to a value.
    pub fn set_variable(&mut self, var: u8, value: u16) {
        unimplemented!()
    }
    /// Restarts the game. The only surviving information is the transcription mode and the fixed
    /// pitch font mode.
    pub fn restart(&mut self) {
        unimplemented!()
    }
    /// Pops the top value off of the stack and returns it.
    pub fn pop_stack(&mut self) -> u16 {
        unimplemented!()
    }
    /// Returns the current stack frame.
    pub fn stack_frame(&self) -> u16 {
        unimplemented!()
    }
    /// Stops execution immediately. Returns from [`run`](ZMachine::run).
    pub fn quit(&mut self) {
        unimplemented!()
    }
    /// Updates the status line without waiting for keyboard input.
    pub fn update_status_line(&mut self) {
        unimplemented!()
    }
    pub fn invoke_routine(&mut self, addr: ByteAddress, retvar: u8) -> RoutineResult {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Fail)]
pub enum ExecuteError {
    #[fail(display = "Invalid opcode {}", _0)]
    InvalidOpcode(u8),
    #[fail(display = "Invalid instruction format at address {}", _0)]
    InvalidInstructionFormat(usize),
}

type ExecuteResult = Result<Action, ExecuteError>;
type RoutineResult = Result<u16, ExecuteError>;

#[derive(Debug)]
enum OpcodeForm {
    Long,
    Short,
    Extended,
    Variable,
}

#[derive(Clone)]
crate enum Action {
    Continue,
    Return(u16),
    Call {
        addr: ByteAddress,
        retvar: Option<u8>,
        args: ArrayVec<[u16; 7]>,
    },
}

#[derive(Debug)]
crate enum OperandsDesc {
    Op0,
    Op1,
    Op2,
    Var,
}

#[derive(Debug, Copy, Clone)]
crate enum Operand {
    LargeConstant(u16),
    SmallConstant(u8),
    Variable(u8),
    Omitted,
}

impl Operand {
    crate fn resolve(self, z: &mut ZMachine) -> Option<Word> {
        match self {
            Operand::LargeConstant(constant) => Some(constant),
            Operand::SmallConstant(constant) => Some(constant as Word),
            Operand::Variable(var) => Some(z.variable(var)),
            Operand::Omitted => None,
        }
    }
}
