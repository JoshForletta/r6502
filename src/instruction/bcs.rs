use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const BCS_RELATIVE: Instruction = Instruction {
    opcode: 0xB0,
    mnemonic: "BCS",
    am: addressing_mode::RELATIVE,
    call: bcs,
};

pub fn bcs(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn bcs_relative() {}
}
