use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const PLA_IMPLIED: Instruction = Instruction {
    opcode: 0x68,
    mnemonic: "PLA",
    am: addressing_mode::IMPLIED,
    call: pla,
};

pub fn pla(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn pla_implied() {}
}