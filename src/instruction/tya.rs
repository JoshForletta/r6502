use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const TYA_IMPLIED: Instruction = Instruction {
    opcode: 0x98,
    mnemonic: "TYA",
    am: addressing_mode::IMPLIED,
    call: tya,
};

pub fn tya(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.a = cpu.y;

    cpu.ps.set(PS::Z, cpu.a == 0);
    cpu.ps.set(PS::N, (cpu.a & 0x80) != 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn tya_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0x98],
            initial_cpu_state: CpuState {
                y: Some(0x00),
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
    fn tya_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0x98],
            initial_cpu_state: CpuState {
                y: Some(0x80),
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
    fn tya() {
        let est = EmulationStateTest {
            instructions: &[0x98],
            initial_cpu_state: CpuState {
                y: Some(0x69),
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
