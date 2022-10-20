use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const BVC_RELATIVE: Instruction = Instruction {
    opcode: 0x50,
    mnemonic: "BVC",
    am: addressing_mode::RELATIVE,
    call: bvc,
};

pub fn bvc(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn bvc_relative() {}
}
