use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const LDA_IMMEDIATE: Instruction = Instruction {
    opcode: 0xA9,
    mnemonic: "LDA",
    am: addressing_mode::IMMEDIATE,
    call: lda,
};

pub const LDA_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xA5,
    mnemonic: "LDA",
    am: addressing_mode::ZERO_PAGE,
    call: lda,
};

pub const LDA_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0xB5,
    mnemonic: "LDA",
    am: addressing_mode::ZERO_PAGE_X,
    call: lda,
};

pub const LDA_ABSOLUTE: Instruction = Instruction {
    opcode: 0xAD,
    mnemonic: "LDA",
    am: addressing_mode::ABSOLUTE,
    call: lda,
};

pub const LDA_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0xBD,
    mnemonic: "LDA",
    am: addressing_mode::ABSOLUTE_X,
    call: lda,
};

pub const LDA_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0xB9,
    mnemonic: "LDA",
    am: addressing_mode::ABSOLUTE_Y,
    call: lda,
};

pub const LDA_INDEXED_INDIRECT: Instruction = Instruction {
    opcode: 0xA1,
    mnemonic: "LDA",
    am: addressing_mode::INDEXED_INDIRECT,
    call: lda,
};

pub const LDA_INDIRECT_INDEXED: Instruction = Instruction {
    opcode: 0xB1,
    mnemonic: "LDA",
    am: addressing_mode::INDIRECT_INDEXED,
    call: lda,
};

pub fn lda(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn lda_immediate() {}
    //
    // #[test]
    // fn lda_zero_page() {}
    //
    // #[test]
    // fn lda_zero_page_x() {}
    //
    // #[test]
    // fn lda_absolute() {}
    //
    // #[test]
    // fn lda_absolute_x() {}
    //
    // #[test]
    // fn lda_absolute_y() {}
    //
    // #[test]
    // fn lda_indexed_indirect() {}
    //
    // #[test]
    // fn lda_indirect_indexed() {}
}
