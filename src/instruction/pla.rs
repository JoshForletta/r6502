use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const PLA_IMPLIED: Instruction = Instruction {
    opcode: 0x68,
    mnemonic: "PLA",
    am: addressing_mode::IMPLIED,
    call: pla,
};

pub fn pla(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.a = cpu.pull()?;

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
    fn pla_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0x68],
            initial_mem: &[(0x0100, 0x00)],
            initial_cpu_state: CpuState {
                sp: Some(0x01),
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
    fn pla_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0x68],
            initial_mem: &[(0x0100, 0x80)],
            initial_cpu_state: CpuState {
                sp: Some(0x01),
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
    fn pla() {
        let est = EmulationStateTest {
            instructions: &[0x68],
            initial_mem: &[(0x0100, 0x69)],
            initial_cpu_state: CpuState {
                sp: Some(0x01),
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
