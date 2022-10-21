use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const ROL_ACCUMULATOR: Instruction = Instruction {
    opcode: 0x2A,
    mnemonic: "ROL",
    am: addressing_mode::ACCUMULATOR,
    call: rol,
};

pub const ROL_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x26,
    mnemonic: "ROL",
    am: addressing_mode::ZERO_PAGE,
    call: rol,
};

pub const ROL_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x36,
    mnemonic: "ROL",
    am: addressing_mode::ZERO_PAGE_X,
    call: rol,
};

pub const ROL_ABSOLUTE: Instruction = Instruction {
    opcode: 0x2E,
    mnemonic: "ROL",
    am: addressing_mode::ABSOLUTE,
    call: rol,
};

pub const ROL_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x3E,
    mnemonic: "ROL",
    am: addressing_mode::ABSOLUTE_X,
    call: rol,
};

pub fn rol(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn rol_accumulator() {}
    //
    // #[test]
    // fn rol_zero_page() {}
    //
    // #[test]
    // fn rol_zero_page_x() {}
    //
    // #[test]
    // fn rol_absolute() {}
    //
    // #[test]
    // fn rol_absolute_x() {}
}
