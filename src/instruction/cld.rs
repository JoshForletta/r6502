use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const CLD_IMPLIED: Instruction = Instruction {
    opcode: 0xD8,
    mnemonic: "CLD",
    am: addressing_mode::IMPLIED,
    call: cld,
};

pub fn cld(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn cld_implied() {}
}
