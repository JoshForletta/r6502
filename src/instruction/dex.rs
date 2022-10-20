use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const DEX_IMPLIED: Instruction = Instruction {
    opcode: 0xCA,
    mnemonic: "DEX",
    am: addressing_mode::IMPLIED,
    call: dex,
};

pub fn dex(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn dex_implied() {}
}
