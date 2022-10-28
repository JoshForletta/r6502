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

pub fn pha(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.push(cpu.a)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};

    #[test]
    fn pha() {
        let est = EmulationStateTest {
            instructions: &[0x48],
            initial_cpu_state: CpuState {
                a: Some(0x69),
                ..Default::default()
            },
            mem_tests: &[(0x0100, 0x69)],
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
