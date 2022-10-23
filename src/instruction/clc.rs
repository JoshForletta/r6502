use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const CLC_IMPLIED: Instruction = Instruction {
    opcode: 0x18,
    mnemonic: "CLC",
    am: addressing_mode::IMPLIED,
    call: clc,
};

pub fn clc(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.ps.remove(PS::C);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn clc() {
        let est = EmulationStateTest {
            instructions: &[0x18],
            initial_cpu_state: CpuState {
                ps: Some(PS::C),
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
