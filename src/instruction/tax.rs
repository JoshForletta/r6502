use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const TAX_IMPLIED: Instruction = Instruction {
    opcode: 0xAA,
    mnemonic: "TAX",
    am: addressing_mode::IMPLIED,
    call: tax,
};

pub fn tax(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.x = cpu.a;

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
    fn tax_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0xAA],
            initial_cpu_state: CpuState {
                a: Some(0x00),
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
    fn tax_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0xAA],
            initial_cpu_state: CpuState {
                a: Some(0x80),
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
    fn tax() {
        let est = EmulationStateTest {
            instructions: &[0xAA],
            initial_cpu_state: CpuState {
                a: Some(0x69),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                x: Some(0x69),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
