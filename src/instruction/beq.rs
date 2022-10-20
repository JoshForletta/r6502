use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const BEQ_RELATIVE: Instruction = Instruction {
    opcode: 0xF0,
    mnemonic: "BEQ",
    am: addressing_mode::RELATIVE,
    call: beq,
};

pub fn beq(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn beq_relative() {}
}
