use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const PHA_IMPLIED: Instruction = Instruction {
    opcode: 0x48,
    mnemonic: "PHA",
    am: addressing_mode::IMPLIED,
    call: pha,
};

pub fn pha(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn pha_implied() {}
}
