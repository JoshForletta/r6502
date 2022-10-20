use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const ASL_ACCUMULATOR: Instruction = Instruction {
    opcode: 0x0A,
    mnemonic: "ASL",
    am: addressing_mode::ACCUMULATOR,
    call: asl,
};

pub const ASL_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x06,
    mnemonic: "ASL",
    am: addressing_mode::ZERO_PAGE,
    call: asl,
};

pub const ASL_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x16,
    mnemonic: "ASL",
    am: addressing_mode::ZERO_PAGE_X,
    call: asl,
};

pub const ASL_ABSOLUTE: Instruction = Instruction {
    opcode: 0x0E,
    mnemonic: "ASL",
    am: addressing_mode::ABSOLUTE,
    call: asl,
};

pub const ASL_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x1E,
    mnemonic: "ASL",
    am: addressing_mode::ABSOLUTE_X,
    call: asl,
};

pub fn asl(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn asl_accumulator() {}

    #[test]
    fn asl_zero_page() {}

    #[test]
    fn asl_zero_page_x() {}

    #[test]
    fn asl_absolute() {}

    #[test]
    fn asl_absolute_x() {}
}
