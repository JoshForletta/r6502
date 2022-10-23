use std::{error::Error, num::Wrapping};

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const DEY_IMPLIED: Instruction = Instruction {
    opcode: 0x88,
    mnemonic: "DEY",
    am: addressing_mode::IMPLIED,
    call: dey,
};

pub fn dey(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.y = (Wrapping(cpu.y) - Wrapping(1)).0;

    cpu.ps.set(PS::Z, cpu.y == 0);
    cpu.ps.set(PS::N, (cpu.y & 0x80) != 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn dey_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0x88],
            initial_cpu_state: CpuState {
                y: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::Z),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn dey_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0x88],
            initial_cpu_state: CpuState {
                y: Some(0x00),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::N),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn dey() {
        let est = EmulationStateTest {
            instructions: &[0x88],
            initial_cpu_state: CpuState {
                y: Some(0x69),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                y: Some(0x68),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
