use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const TXS_IMPLIED: Instruction = Instruction {
    opcode: 0x9A,
    mnemonic: "TXS",
    am: addressing_mode::IMPLIED,
    call: txs,
};

pub fn txs(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let _target = (am.call)(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn txs_implied() {}
}
