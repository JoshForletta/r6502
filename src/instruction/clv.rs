use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const CLV_IMPLIED: Instruction = Instruction {
    opcode: 0xB8,
    mnemonic: "CLV",
    am: addressing_mode::IMPLIED,
    call: clv,
};

pub fn clv(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn clv_implied() {}
}
