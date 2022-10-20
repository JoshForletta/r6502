use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const LSR_ACCUMULATOR: Instruction = Instruction {
    opcode: 0x4A,
    mnemonic: "LSR",
    am: addressing_mode::ACCUMULATOR,
    call: lsr,
};

pub const LSR_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x46,
    mnemonic: "LSR",
    am: addressing_mode::ZERO_PAGE,
    call: lsr,
};

pub const LSR_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x56,
    mnemonic: "LSR",
    am: addressing_mode::ZERO_PAGE_X,
    call: lsr,
};

pub const LSR_ABSOLUTE: Instruction = Instruction {
    opcode: 0x4E,
    mnemonic: "LSR",
    am: addressing_mode::ABSOLUTE,
    call: lsr,
};

pub const LSR_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x5E,
    mnemonic: "LSR",
    am: addressing_mode::ABSOLUTE_X,
    call: lsr,
};

pub fn lsr(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn lsr_accumulator() {}
    //
    // #[test]
    // fn lsr_zero_page() {}
    //
    // #[test]
    // fn lsr_zero_page_x() {}
    //
    // #[test]
    // fn lsr_absolute() {}
    //
    // #[test]
    // fn lsr_absolute_x() {}
}
