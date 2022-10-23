use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const CLV_IMPLIED: Instruction = Instruction {
    opcode: 0xB8,
    mnemonic: "CLV",
    am: addressing_mode::IMPLIED,
    call: clv,
};

pub fn clv(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.ps.remove(PS::V);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn clv() {
        let est = EmulationStateTest {
            instructions: &[0xB8],
            initial_cpu_state: CpuState {
                ps: Some(PS::V),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::empty()),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
