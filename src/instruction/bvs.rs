use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const BVS_RELATIVE: Instruction = Instruction {
    opcode: 0x70,
    mnemonic: "BVS",
    am: addressing_mode::RELATIVE,
    call: bvs,
};

pub fn bvs(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn bvs_relative() {}
}
