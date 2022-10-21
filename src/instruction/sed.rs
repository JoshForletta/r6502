use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const SED_IMPLIED: Instruction = Instruction {
    opcode: 0xF8,
    mnemonic: "SED",
    am: addressing_mode::IMPLIED,
    call: sed,
};

pub fn sed(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn sed_implied() {}
}
