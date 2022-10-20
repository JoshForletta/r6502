use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const BVS_RELATIVE: Instruction = Instruction {
    opcode: 0x70,
    mnemonic: "BVS",
    am: addressing_mode::RELATIVE,
    call: bvs,
};

pub fn bvs(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn bvs_relative() {}
}
