use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const CPY_IMMEDIATE: Instruction = Instruction {
    opcode: 0xC0,
    mnemonic: "CPY",
    am: addressing_mode::IMMEDIATE,
    call: cpy,
};

pub const CPY_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xC4,
    mnemonic: "CPY",
    am: addressing_mode::ZERO_PAGE,
    call: cpy,
};

pub const CPY_ABSOLUTE: Instruction = Instruction {
    opcode: 0xCC,
    mnemonic: "CPY",
    am: addressing_mode::ABSOLUTE,
    call: cpy,
};

pub fn cpy(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn cpy_immediate() {}
    //
    // #[test]
    // fn cpy_zero_page() {}
    //
    // #[test]
    // fn cpy_absolute() {}
}
