use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const BCC_RELATIVE: Instruction = Instruction {
    opcode: 0x90,
    mnemonic: "BCC",
    am: addressing_mode::RELATIVE,
    call: bcc,
};

pub fn bcc(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn bcc_relative() {}
}
