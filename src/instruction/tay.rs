use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const TAY_IMPLIED: Instruction = Instruction {
    opcode: 0xA8,
    mnemonic: "TAY",
    am: addressing_mode::IMPLIED,
    call: tay,
};

pub fn tay(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn tay_implied() {}
}
