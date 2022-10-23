use std::{error::Error, num::Wrapping};

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const DEC_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xC6,
    mnemonic: "DEC",
    am: addressing_mode::ZERO_PAGE,
    call: dec,
};

pub const DEC_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0xD6,
    mnemonic: "DEC",
    am: addressing_mode::ZERO_PAGE_X,
    call: dec,
};

pub const DEC_ABSOLUTE: Instruction = Instruction {
    opcode: 0xCE,
    mnemonic: "DEC",
    am: addressing_mode::ABSOLUTE,
    call: dec,
};

pub const DEC_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0xDE,
    mnemonic: "DEC",
    am: addressing_mode::ABSOLUTE_X,
    call: dec,
};

pub fn dec(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let data = (am.call)(cpu)?;

    *data = (Wrapping(*data) - Wrapping(1)).0;

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
    fn dec_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0xC6, 0x02, 0x01],
            test_cpu_state: CpuState {
                ps: Some(PS::Z),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn dec_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0xC6, 0x02, 0x00],
            test_cpu_state: CpuState {
                ps: Some(PS::N),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn dec() {
        let est = EmulationStateTest {
            instructions: &[0xC6, 0x02, 0x69],
            mem_tests: &[(0x0002, 0x68)],
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
