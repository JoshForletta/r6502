use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const CLD_IMPLIED: Instruction = Instruction {
    opcode: 0xD8,
    mnemonic: "CLD",
    am: addressing_mode::IMPLIED,
    call: cld,
};

pub fn cld(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn cld_implied() {}
}
