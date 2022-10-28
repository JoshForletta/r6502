use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const LSR_ACCUMULATOR: Instruction = Instruction {
    opcode: 0x4A,
    mnemonic: "LSR",
    am: addressing_mode::ACCUMULATOR,
    call: lsr,
};

pub const LSR_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x46,
    mnemonic: "LSR",
    am: addressing_mode::ZERO_PAGE,
    call: lsr,
};

pub const LSR_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x56,
    mnemonic: "LSR",
    am: addressing_mode::ZERO_PAGE_X,
    call: lsr,
};

pub const LSR_ABSOLUTE: Instruction = Instruction {
    opcode: 0x4E,
    mnemonic: "LSR",
    am: addressing_mode::ABSOLUTE,
    call: lsr,
};

pub const LSR_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x5E,
    mnemonic: "LSR",
    am: addressing_mode::ABSOLUTE_X,
    call: lsr,
};

pub fn lsr(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let target = (am.call)(cpu)?;
    let data = (*target as u16) << 7;

    let (carry, data) = (data as u8, (data >> 8) as u8);

    *target = data;

    cpu.ps.set(PS::Z, data == 0);
    cpu.ps.set(PS::C, carry != 0);
    cpu.ps.set(PS::N, false);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn lsr_carry_flag() {
        let est = EmulationStateTest {
            instructions: &[0x46, 0x02, 0xF1],
            test_cpu_state: CpuState {
                ps: Some(PS::C),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn lsr_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0x46, 0x02, 0x00],
            test_cpu_state: CpuState {
                ps: Some(PS::Z),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn lsr() {
        let est = EmulationStateTest {
            instructions: &[0x4A],
            initial_cpu_state: CpuState {
                a: Some(0xFF),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x7F),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
