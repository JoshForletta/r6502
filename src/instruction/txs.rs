use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const TXS_IMPLIED: Instruction = Instruction {
    opcode: 0x9A,
    mnemonic: "TXS",
    am: addressing_mode::IMPLIED,
    call: txs,
};

pub fn txs(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn txs_implied() {}
}
