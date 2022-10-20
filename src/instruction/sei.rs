use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const SEI_IMPLIED: Instruction = Instruction {
    opcode: 0x78,
    mnemonic: "SEI",
    am: addressing_mode::IMPLIED,
    call: sei,
};

pub fn sei(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn sei_implied() {}
}
