use std::{error::Error, num::Wrapping};

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const DEX_IMPLIED: Instruction = Instruction {
    opcode: 0xCA,
    mnemonic: "DEX",
    am: addressing_mode::IMPLIED,
    call: dex,
};

pub fn dex(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.x = (Wrapping(cpu.x) - Wrapping(1)).0;

    cpu.ps.set(PS::Z, cpu.x == 0);
    cpu.ps.set(PS::N, (cpu.x & 0x80) != 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn dex_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0xCA],
            initial_cpu_state: CpuState {
                x: Some(0x01),
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
    fn dex_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0xCA],
            initial_cpu_state: CpuState {
                x: Some(0x00),
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
    fn dex() {
        let est = EmulationStateTest {
            instructions: &[0xCA],
            initial_cpu_state: CpuState {
                x: Some(0x69),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                x: Some(0x68),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
