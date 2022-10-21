use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const INC_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xE6,
    mnemonic: "INC",
    am: addressing_mode::ZERO_PAGE,
    call: inc,
};

pub const INC_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0xF6,
    mnemonic: "INC",
    am: addressing_mode::ZERO_PAGE_X,
    call: inc,
};

pub const INC_ABSOLUTE: Instruction = Instruction {
    opcode: 0xEE,
    mnemonic: "INC",
    am: addressing_mode::ABSOLUTE,
    call: inc,
};

pub const INC_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0xFE,
    mnemonic: "INC",
    am: addressing_mode::ABSOLUTE_X,
    call: inc,
};

pub fn inc(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn inc_zero_page() {}
    //
    // #[test]
    // fn inc_zero_page_x() {}
    //
    // #[test]
    // fn inc_absolute() {}
    //
    // #[test]
    // fn inc_absolute_x() {}
}
