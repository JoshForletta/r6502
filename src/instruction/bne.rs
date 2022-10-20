use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const BNE_RELATIVE: Instruction = Instruction {
    opcode: 0xD0,
    mnemonic: "BNE",
    am: addressing_mode::RELATIVE,
    call: bne,
};

pub fn bne(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn bne_relative() {}
}
