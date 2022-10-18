use std::{error::Error, fmt::Debug};

use crate::{
    error::{BusError, InstructionError},
    R6502,
};

type AddrMode = fn(&mut R6502) -> Result<&mut u8, BusError>;

pub const INSTRUCTION_SET: &[Instruction] = &[AND_IMM, LDA_IMM];

#[derive(Clone, Copy)]
pub struct Instruction {
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub addr_mode: AddrMode,
    ins: fn(&mut R6502, AddrMode) -> Result<(), Box<dyn Error>>,
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
        match (self.ins)(cpu, self.addr_mode) {
            Ok(_) => Ok(()),
            Err(e) => Err(InstructionError {
                instruction: self.clone(),
                error: e,
            }),
        }
    }
}

pub fn immediate(cpu: &mut R6502) -> Result<&mut u8, BusError> {
    let addr = cpu.read_word()?;
    cpu.bus.read_mut(addr)
}

pub const AND_IMM: Instruction = Instruction {
    opcode: 0x00,
    mnemonic: "AND",
    addr_mode: immediate,
    ins: and,
};

pub const LDA_IMM: Instruction = Instruction {
    opcode: 0xA9,
    mnemonic: "LDA",
    addr_mode: immediate,
    ins: lda,
};

pub fn lda(cpu: &mut R6502, addr_mode: AddrMode) -> Result<(), Box<dyn Error>> {
    cpu.a = *addr_mode(cpu)?;

    Ok(())
}

pub fn and(cpu: &mut R6502, addr_mode: AddrMode) -> Result<(), Box<dyn Error>> {
    let data = addr_mode(cpu)?;

    cpu.a &= *data;

    Ok(())
}
