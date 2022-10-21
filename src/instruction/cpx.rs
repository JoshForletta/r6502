use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const CPX_IMMEDIATE: Instruction = Instruction {
    opcode: 0xE0,
    mnemonic: "CPX",
    am: addressing_mode::IMMEDIATE,
    call: cpx,
};

pub const CPX_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xE4,
    mnemonic: "CPX",
    am: addressing_mode::ZERO_PAGE,
    call: cpx,
};

pub const CPX_ABSOLUTE: Instruction = Instruction {
    opcode: 0xEC,
    mnemonic: "CPX",
    am: addressing_mode::ABSOLUTE,
    call: cpx,
};

pub fn cpx(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn cpx_immediate() {}
    //
    // #[test]
    // fn cpx_zero_page() {}
    //
    // #[test]
    // fn cpx_absolute() {}
}
