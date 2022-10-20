use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const DEC_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xC6,
    mnemonic: "DEC",
    am: addressing_mode::ZERO_PAGE,
    call: dec,
};

pub const DEC_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0xD6,
    mnemonic: "DEC",
    am: addressing_mode::ZERO_PAGE_X,
    call: dec,
};

pub const DEC_ABSOLUTE: Instruction = Instruction {
    opcode: 0xCE,
    mnemonic: "DEC",
    am: addressing_mode::ABSOLUTE,
    call: dec,
};

pub const DEC_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0xDE,
    mnemonic: "DEC",
    am: addressing_mode::ABSOLUTE_X,
    call: dec,
};

pub fn dec(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn dec_zero_page() {}

    #[test]
    fn dec_zero_page_x() {}

    #[test]
    fn dec_absolute() {}

    #[test]
    fn dec_absolute_x() {}
}
