use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const TYA_IMPLIED: Instruction = Instruction {
    opcode: 0x98,
    mnemonic: "TYA",
    am: addressing_mode::IMPLIED,
    call: tya,
};

pub fn tya(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn tya_implied() {}
}
