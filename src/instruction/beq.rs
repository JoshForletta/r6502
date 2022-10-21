use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const BEQ_RELATIVE: Instruction = Instruction {
    opcode: 0xF0,
    mnemonic: "BEQ",
    am: addressing_mode::RELATIVE,
    call: beq,
};

pub fn beq(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn beq_relative() {}
}
