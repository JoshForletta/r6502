use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const ROR_ACCUMULATOR: Instruction = Instruction {
    opcode: 0x6A,
    mnemonic: "ROR",
    am: addressing_mode::ACCUMULATOR,
    call: ror,
};

pub const ROR_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x66,
    mnemonic: "ROR",
    am: addressing_mode::ZERO_PAGE,
    call: ror,
};

pub const ROR_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x76,
    mnemonic: "ROR",
    am: addressing_mode::ZERO_PAGE_X,
    call: ror,
};

pub const ROR_ABSOLUTE: Instruction = Instruction {
    opcode: 0x6E,
    mnemonic: "ROR",
    am: addressing_mode::ABSOLUTE,
    call: ror,
};

pub const ROR_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x7E,
    mnemonic: "ROR",
    am: addressing_mode::ABSOLUTE_X,
    call: ror,
};

pub fn ror(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let carry = cpu.ps.contains(PS::C);
    let target = (am.call)(cpu)?;
    let data = ((*target as u16) << 7) | ((carry as u16) << 15);

    let (carry, data) = (data as u8, (data >> 8) as u8);

    *target = data;

    cpu.ps.set(PS::Z, data == 0);
    cpu.ps.set(PS::C, carry != 0);
    cpu.ps.set(PS::N, (data & 0x80) != 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn ror_carry_flag() {
        let est = EmulationStateTest {
            instructions: &[0x6A],
            initial_cpu_state: CpuState {
                a: Some(0xF1),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::C),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn ror_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0x6A],
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
    fn ror_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0x6A],
            initial_cpu_state: CpuState {
                ps: Some(PS::C),
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
    fn ror() {
        let est = EmulationStateTest {
            instructions: &[0x6A],
            initial_cpu_state: CpuState {
                a: Some(0xF0),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x78),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn ror_with_carry() {
        let est = EmulationStateTest {
            instructions: &[0x6A],
            initial_cpu_state: CpuState {
                ps: Some(PS::C),
                a: Some(0xF0),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0xF8),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
