use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const BIT_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x24,
    mnemonic: "BIT",
    am: addressing_mode::ZERO_PAGE,
    call: bit,
};

pub const BIT_ABSOLUTE: Instruction = Instruction {
    opcode: 0x2C,
    mnemonic: "BIT",
    am: addressing_mode::ABSOLUTE,
    call: bit,
};

pub fn bit(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn bit_zero_page() {}
    //
    // #[test]
    // fn bit_absolute() {}
}
