use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const ASL_ACCUMULATOR: Instruction = Instruction {
    opcode: 0x0A,
    mnemonic: "ASL",
    am: addressing_mode::ACCUMULATOR,
    call: asl,
};

pub const ASL_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x06,
    mnemonic: "ASL",
    am: addressing_mode::ZERO_PAGE,
    call: asl,
};

pub const ASL_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x16,
    mnemonic: "ASL",
    am: addressing_mode::ZERO_PAGE_X,
    call: asl,
};

pub const ASL_ABSOLUTE: Instruction = Instruction {
    opcode: 0x0E,
    mnemonic: "ASL",
    am: addressing_mode::ABSOLUTE,
    call: asl,
};

pub const ASL_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x1E,
    mnemonic: "ASL",
    am: addressing_mode::ABSOLUTE_X,
    call: asl,
};

pub fn asl(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let target = (am.call)(cpu)?;
    let mut data = target.clone() as u16;

    data = data << 1;

    let (carry, data) = ((data >> 8) as u8, data as u8);

    *target = data;

    cpu.ps.set(PS::C, carry != 0);
    cpu.ps.set(PS::Z, data == 0);
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
    fn asl_carry_flag() {
        let est = EmulationStateTest {
            instructions: &[0x0A],
            initial_cpu_state: CpuState {
                a: Some(0xA0),
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
    fn asl_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0x0A],
            initial_cpu_state: CpuState {
                a: Some(0x80),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::Z | PS::C),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn asl_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0x0A],
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
    fn asl() {
        let est = EmulationStateTest {
            instructions: &[0x0A],
            initial_cpu_state: CpuState {
                a: Some(0xA0),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x40),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
