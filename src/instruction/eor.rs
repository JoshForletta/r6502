use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const EOR_IMMEDIATE: Instruction = Instruction {
    opcode: 0x49,
    mnemonic: "EOR",
    am: addressing_mode::IMMEDIATE,
    call: eor,
};

pub const EOR_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x45,
    mnemonic: "EOR",
    am: addressing_mode::ZERO_PAGE,
    call: eor,
};

pub const EOR_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x55,
    mnemonic: "EOR",
    am: addressing_mode::ZERO_PAGE_X,
    call: eor,
};

pub const EOR_ABSOLUTE: Instruction = Instruction {
    opcode: 0x4D,
    mnemonic: "EOR",
    am: addressing_mode::ABSOLUTE,
    call: eor,
};

pub const EOR_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x5D,
    mnemonic: "EOR",
    am: addressing_mode::ABSOLUTE_X,
    call: eor,
};

pub const EOR_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0x59,
    mnemonic: "EOR",
    am: addressing_mode::ABSOLUTE_Y,
    call: eor,
};

pub const EOR_INDEXED_INDIRECT: Instruction = Instruction {
    opcode: 0x41,
    mnemonic: "EOR",
    am: addressing_mode::INDEXED_INDIRECT,
    call: eor,
};

pub const EOR_INDIRECT_INDEXED: Instruction = Instruction {
    opcode: 0x51,
    mnemonic: "EOR",
    am: addressing_mode::INDIRECT_INDEXED,
    call: eor,
};

pub fn eor(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn eor_immediate() {}

    #[test]
    fn eor_zero_page() {}

    #[test]
    fn eor_zero_page_x() {}

    #[test]
    fn eor_absolute() {}

    #[test]
    fn eor_absolute_x() {}

    #[test]
    fn eor_absolute_y() {}

    #[test]
    fn eor_indexed_indirect() {}

    #[test]
    fn eor_indirect_indexed() {}
}
