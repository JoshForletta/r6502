use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const STX_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x86,
    mnemonic: "STX",
    am: addressing_mode::ZERO_PAGE,
    call: stx,
};

pub const STX_ZERO_PAGE_Y: Instruction = Instruction {
    opcode: 0x96,
    mnemonic: "STX",
    am: addressing_mode::ZERO_PAGE_Y,
    call: stx,
};

pub const STX_ABSOLUTE: Instruction = Instruction {
    opcode: 0x8E,
    mnemonic: "STX",
    am: addressing_mode::ABSOLUTE,
    call: stx,
};

pub fn stx(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn stx_zero_page() {}
    //
    // #[test]
    // fn stx_zero_page_y() {}
    //
    // #[test]
    // fn stx_absolute() {}
}
