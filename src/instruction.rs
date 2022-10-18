// use crate::{core::BitWidth, error::InstructionError, R6502, bus::Bus};
//
// pub struct Instruction<Cpu, Opcode>
// where
//     Opcode: BitWidth
// {
//     pub opcode: Opcode,
//     pub mnemonic: &'static str,
//     pub exec: fn(&mut Cpu) -> Result<(), InstructionError>,
// }
//
// pub const LDA: Instruction<R6502<dyn Bus>, u8> = Instruction<R6502<dyn Bus>, u8> {
//     opcode: 0x
// }
