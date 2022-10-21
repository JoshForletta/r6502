use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const JSR_ABSOLUTE: Instruction = Instruction {
    opcode: 0x20,
    mnemonic: "JSR",
    am: addressing_mode::ABSOLUTE,
    call: jsr,
};

pub fn jsr(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn jsr_absolute() {}
}
