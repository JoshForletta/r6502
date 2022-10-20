use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const SEC_IMPLIED: Instruction = Instruction {
    opcode: 0x38,
    mnemonic: "SEC",
    am: addressing_mode::IMPLIED,
    call: sec,
};

pub fn sec(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn sec_implied() {}
}
