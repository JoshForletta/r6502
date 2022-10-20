use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const BPL_RELATIVE: Instruction = Instruction {
    opcode: 0x10,
    mnemonic: "BPL",
    am: addressing_mode::RELATIVE,
    call: bpl,
};

pub fn bpl(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn bpl_relative() {}
}
