use std::{error::Error, num::Wrapping};

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const INC_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xE6,
    mnemonic: "INC",
    am: addressing_mode::ZERO_PAGE,
    call: inc,
};

pub const INC_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0xF6,
    mnemonic: "INC",
    am: addressing_mode::ZERO_PAGE_X,
    call: inc,
};

pub const INC_ABSOLUTE: Instruction = Instruction {
    opcode: 0xEE,
    mnemonic: "INC",
    am: addressing_mode::ABSOLUTE,
    call: inc,
};

pub const INC_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0xFE,
    mnemonic: "INC",
    am: addressing_mode::ABSOLUTE_X,
    call: inc,
};

pub fn inc(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let data = (am.call)(cpu)?;

    *data = (Wrapping(*data) + Wrapping(1)).0;

    let data = *data;

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
    fn inc_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0xE6, 0x02, 0xFF],
            test_cpu_state: CpuState {
                ps: Some(PS::Z),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn inc_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0xE6, 0x02, 0x7F],
            test_cpu_state: CpuState {
                ps: Some(PS::N),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn inc() {
        let est = EmulationStateTest {
            instructions: &[0xE6, 0x02, 0x69],
            mem_tests: &[(0x0002, 0x6A)],
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
