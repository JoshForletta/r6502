use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const TSX_IMPLIED: Instruction = Instruction {
    opcode: 0xBA,
    mnemonic: "TSX",
    am: addressing_mode::IMPLIED,
    call: tsx,
};

pub fn tsx(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn tsx_implied() {}
}
