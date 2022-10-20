use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const STY_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x84,
    mnemonic: "STY",
    am: addressing_mode::ZERO_PAGE,
    call: sty,
};

pub const STY_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x94,
    mnemonic: "STY",
    am: addressing_mode::ZERO_PAGE_X,
    call: sty,
};

pub const STY_ABSOLUTE: Instruction = Instruction {
    opcode: 0x8C,
    mnemonic: "STY",
    am: addressing_mode::ABSOLUTE,
    call: sty,
};

pub fn sty(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn sty_zero_page() {}
    //
    // #[test]
    // fn sty_zero_page_x() {}
    //
    // #[test]
    // fn sty_absolute() {}
}
