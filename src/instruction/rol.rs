use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const ROL_ACCUMULATOR: Instruction = Instruction {
    opcode: 0x2A,
    mnemonic: "ROL",
    am: addressing_mode::ACCUMULATOR,
    call: rol,
};

pub const ROL_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x26,
    mnemonic: "ROL",
    am: addressing_mode::ZERO_PAGE,
    call: rol,
};

pub const ROL_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x36,
    mnemonic: "ROL",
    am: addressing_mode::ZERO_PAGE_X,
    call: rol,
};

pub const ROL_ABSOLUTE: Instruction = Instruction {
    opcode: 0x2E,
    mnemonic: "ROL",
    am: addressing_mode::ABSOLUTE,
    call: rol,
};

pub const ROL_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x3E,
    mnemonic: "ROL",
    am: addressing_mode::ABSOLUTE_X,
    call: rol,
};

pub fn rol(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let carry = cpu.ps.contains(PS::C);
    let target = (am.call)(cpu)?;
    let data = ((*target as u16) << 1) | carry as u16;

    let (carry, data) = ((data >> 8) as u8, data as u8);

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
    fn rol_carry_flag() {
        let est = EmulationStateTest {
            instructions: &[0x2A],
            initial_cpu_state: CpuState {
                a: Some(0x81),
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
    fn rol_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0x2A],
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
    fn rol_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0x2A],
            initial_cpu_state: CpuState {
                a: Some(0x40),
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
    fn rol() {
        let est = EmulationStateTest {
            instructions: &[0x2A],
            initial_cpu_state: CpuState {
                a: Some(0xF0),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0xE0),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn rol_with_carry() {
        let est = EmulationStateTest {
            instructions: &[0x2A],
            initial_cpu_state: CpuState {
                ps: Some(PS::C),
                a: Some(0xF0),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0xE1),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
