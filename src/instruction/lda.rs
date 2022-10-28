use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const LDA_IMMEDIATE: Instruction = Instruction {
    opcode: 0xA9,
    mnemonic: "LDA",
    am: addressing_mode::IMMEDIATE,
    call: lda,
};

pub const LDA_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xA5,
    mnemonic: "LDA",
    am: addressing_mode::ZERO_PAGE,
    call: lda,
};

pub const LDA_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0xB5,
    mnemonic: "LDA",
    am: addressing_mode::ZERO_PAGE_X,
    call: lda,
};

pub const LDA_ABSOLUTE: Instruction = Instruction {
    opcode: 0xAD,
    mnemonic: "LDA",
    am: addressing_mode::ABSOLUTE,
    call: lda,
};

pub const LDA_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0xBD,
    mnemonic: "LDA",
    am: addressing_mode::ABSOLUTE_X,
    call: lda,
};

pub const LDA_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0xB9,
    mnemonic: "LDA",
    am: addressing_mode::ABSOLUTE_Y,
    call: lda,
};

pub const LDA_INDEXED_INDIRECT: Instruction = Instruction {
    opcode: 0xA1,
    mnemonic: "LDA",
    am: addressing_mode::INDEXED_INDIRECT,
    call: lda,
};

pub const LDA_INDIRECT_INDEXED: Instruction = Instruction {
    opcode: 0xB1,
    mnemonic: "LDA",
    am: addressing_mode::INDIRECT_INDEXED,
    call: lda,
};

pub fn lda(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.a = *(am.call)(cpu)?;

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
    fn lda_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0xA9, 0x00],
            test_cpu_state: CpuState {
                ps: Some(PS::Z),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn lda_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0xA9, 0x80],
            test_cpu_state: CpuState {
                ps: Some(PS::N),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn lda() {
        let est = EmulationStateTest {
            instructions: &[0xA9, 0x69],
            test_cpu_state: CpuState {
                a: Some(0x69),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
