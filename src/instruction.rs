use std::{error::Error, fmt::Debug};

use crate::{
    addressing_mode::{AddressingMode, AmFn},
    error::InstructionError,
    R6502,
};

pub const INSTRUCTION_SET: &[Instruction] = &[];

#[derive(Clone, Copy)]
pub struct Instruction {
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub am: AddressingMode,
    ins: fn(&mut R6502, AmFn) -> Result<(), Box<dyn Error>>,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instruction")
            .field("opcode", &format_args!("${:02X}", self.opcode))
            .field("mneumonic", &self.mnemonic)
            .finish()
    }
}

impl Instruction {
    pub fn exec(&self, cpu: &mut R6502) -> Result<(), InstructionError> {
        match (self.ins)(cpu, self.am.call) {
            Ok(_) => Ok(()),
            Err(e) => Err(InstructionError {
                instruction: self.clone(),
                error: e,
            }),
        }
    }
}
