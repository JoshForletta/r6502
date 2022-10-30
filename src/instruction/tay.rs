use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const TAY_IMPLIED: Instruction = Instruction {
    opcode: 0xA8,
    mnemonic: "TAY",
    am: addressing_mode::IMPLIED,
    call: tay,
};

pub fn tay(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.y = cpu.a;

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
    fn tay_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0xA8],
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
    fn tay_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0xA8],
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
    fn tay() {
        let est = EmulationStateTest {
            instructions: &[0xA8],
            initial_cpu_state: CpuState {
                a: Some(0x69),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                y: Some(0x69),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
