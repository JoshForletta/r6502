use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const TXA_IMPLIED: Instruction = Instruction {
    opcode: 0x8A,
    mnemonic: "TXA",
    am: addressing_mode::IMPLIED,
    call: txa,
};

pub fn txa(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.a = cpu.x;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};

    #[test]
    fn txa() {
        let est = EmulationStateTest {
            instructions: &[0x8A],
            initial_cpu_state: CpuState {
                x: Some(0x69),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x69),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
