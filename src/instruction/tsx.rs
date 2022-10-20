use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const TSX_IMPLIED: Instruction = Instruction {
    opcode: 0xBA,
    mnemonic: "TSX",
    am: addressing_mode::IMPLIED,
    call: tsx,
};

pub fn tsx(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn tsx_implied() {}
}
