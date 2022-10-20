use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const INX_IMPLIED: Instruction = Instruction {
    opcode: 0xE8,
    mnemonic: "INX",
    am: addressing_mode::IMPLIED,
    call: inx,
};

pub fn inx(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn inx_implied() {}
}
