use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const NOP_IMPLIED: Instruction = Instruction {
    opcode: 0xEA,
    mnemonic: "NOP",
    am: addressing_mode::IMPLIED,
    call: nop,
};

pub fn nop(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn nop_implied() {}
}
