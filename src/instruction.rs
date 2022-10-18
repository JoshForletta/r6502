// use crate::{core::BitWidth, error::InstructionError, R6502, bus::Bus};
//
// type AddrMode = fn(&mut R6502<dyn Bus>) -> &mut u8;
//
// pub struct Instruction
// {
//     pub opcode: u8,
//     pub mnemonic: &'static str,
//     pub addr_mode: AddrMode,
//     pub exec: fn(&mut R6502<dyn Bus>, AddrMode) -> Result<(), InstructionError>,
// }
//
// pub fn immediate(cpu: &mut R6502<dyn Bus>) -> &mut u8 {
//     let addr = cpu.read_word();
//     cpu.get_mut(addr)
// }
//
// pub const AND: Instruction = Instruction {
//     opcode: 0x00,
//     mnemonic: "BRK",
//     addr_mode: immediate,
//     exec: brk,
// };
//
// pub fn brk(cpu: &mut R6502<dyn Bus>, addr_mode: AddrMode) -> Result<(), InstructionError> {
//     Ok(())
// }
