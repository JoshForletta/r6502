use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const NOP_IMPLIED: Instruction = Instruction {
    opcode: 0xEA,
    mnemonic: "NOP",
    am: addressing_mode::IMPLIED,
    call: nop,
};

pub fn nop(_cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};

    #[test]
    fn nop() {
        let est = EmulationStateTest {
            instructions: &[0xEA],
            test_cpu_state: CpuState {
                pc: Some(0x01),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
