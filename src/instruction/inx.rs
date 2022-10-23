use std::{error::Error, num::Wrapping};

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const INX_IMPLIED: Instruction = Instruction {
    opcode: 0xE8,
    mnemonic: "INX",
    am: addressing_mode::IMPLIED,
    call: inx,
};

pub fn inx(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.x = (Wrapping(cpu.x) + Wrapping(1)).0;

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
    fn inx_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0xE8],
            initial_cpu_state: CpuState {
                x: Some(0xFF),
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
    fn inx_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0xE8],
            initial_cpu_state: CpuState {
                x: Some(0x7F),
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
    fn inx() {
        let est = EmulationStateTest {
            instructions: &[0xE8],
            initial_cpu_state: CpuState {
                x: Some(0x69),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                x: Some(0x6A),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
