use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const LDY_IMMEDIATE: Instruction = Instruction {
    opcode: 0xA0,
    mnemonic: "LDY",
    am: addressing_mode::IMMEDIATE,
    call: ldy,
};

pub const LDY_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xA4,
    mnemonic: "LDY",
    am: addressing_mode::ZERO_PAGE,
    call: ldy,
};

pub const LDY_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0xB4,
    mnemonic: "LDY",
    am: addressing_mode::ZERO_PAGE_X,
    call: ldy,
};

pub const LDY_ABSOLUTE: Instruction = Instruction {
    opcode: 0xAC,
    mnemonic: "LDY",
    am: addressing_mode::ABSOLUTE,
    call: ldy,
};

pub const LDY_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0xBC,
    mnemonic: "LDY",
    am: addressing_mode::ABSOLUTE_X,
    call: ldy,
};

pub fn ldy(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.y = *(am.call)(cpu)?;

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
    fn ldy_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0xA0, 0x00],
            test_cpu_state: CpuState {
                ps: Some(PS::Z),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn ldy_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0xA0, 0x80],
            test_cpu_state: CpuState {
                ps: Some(PS::N),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn ldy() {
        let est = EmulationStateTest {
            instructions: &[0xA0, 0x69],
            test_cpu_state: CpuState {
                y: Some(0x69),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
