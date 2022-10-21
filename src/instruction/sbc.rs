use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const SBC_IMMEDIATE: Instruction = Instruction {
    opcode: 0xE9,
    mnemonic: "SBC",
    am: addressing_mode::IMMEDIATE,
    call: sbc,
};

pub const SBC_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xE5,
    mnemonic: "SBC",
    am: addressing_mode::ZERO_PAGE,
    call: sbc,
};

pub const SBC_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0xF5,
    mnemonic: "SBC",
    am: addressing_mode::ZERO_PAGE_X,
    call: sbc,
};

pub const SBC_ABSOLUTE: Instruction = Instruction {
    opcode: 0xED,
    mnemonic: "SBC",
    am: addressing_mode::ABSOLUTE,
    call: sbc,
};

pub const SBC_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0xFD,
    mnemonic: "SBC",
    am: addressing_mode::ABSOLUTE_X,
    call: sbc,
};

pub const SBC_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0xF9,
    mnemonic: "SBC",
    am: addressing_mode::ABSOLUTE_Y,
    call: sbc,
};

pub const SBC_INDEXED_INDIRECT: Instruction = Instruction {
    opcode: 0xE1,
    mnemonic: "SBC",
    am: addressing_mode::INDEXED_INDIRECT,
    call: sbc,
};

pub const SBC_INDIRECT_INDEXED: Instruction = Instruction {
    opcode: 0xF1,
    mnemonic: "SBC",
    am: addressing_mode::INDIRECT_INDEXED,
    call: sbc,
};

pub fn sbc(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn sbc_immediate() {}
    //
    // #[test]
    // fn sbc_zero_page() {}
    //
    // #[test]
    // fn sbc_zero_page_x() {}
    //
    // #[test]
    // fn sbc_absolute() {}
    //
    // #[test]
    // fn sbc_absolute_x() {}
    //
    // #[test]
    // fn sbc_absolute_y() {}
    //
    // #[test]
    // fn sbc_indexed_indirect() {}
    //
    // #[test]
    // fn sbc_indirect_indexed() {}
}
