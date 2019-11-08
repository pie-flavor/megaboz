use super::*;
use crate::*;
use opcodes::ext::*;

impl ZMachine {
    fn execute_ext(
        &mut self,
        addr: &mut ByteAddress,
        desc: OperandsDesc,
        opcode: u8,
    ) -> ExecuteResult {
        unimplemented!()
    }
}
