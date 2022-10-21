use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const LDX_IMMEDIATE: Instruction = Instruction {
    opcode: 0xA2,
    mnemonic: "LDX",
    am: addressing_mode::IMMEDIATE,
    call: ldx,
};

pub const LDX_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xA6,
    mnemonic: "LDX",
    am: addressing_mode::ZERO_PAGE,
    call: ldx,
};

pub const LDX_ZERO_PAGE_Y: Instruction = Instruction {
    opcode: 0xB6,
    mnemonic: "LDX",
    am: addressing_mode::ZERO_PAGE_Y,
    call: ldx,
};

pub const LDX_ABSOLUTE: Instruction = Instruction {
    opcode: 0xAE,
    mnemonic: "LDX",
    am: addressing_mode::ABSOLUTE,
    call: ldx,
};

pub const LDX_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0xBE,
    mnemonic: "LDX",
    am: addressing_mode::ABSOLUTE_Y,
    call: ldx,
};

pub fn ldx(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn ldx_immediate() {}
    //
    // #[test]
    // fn ldx_zero_page() {}
    //
    // #[test]
    // fn ldx_zero_page_y() {}
    //
    // #[test]
    // fn ldx_absolute() {}
    //
    // #[test]
    // fn ldx_absolute_y() {}
}
