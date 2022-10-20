use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
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

pub fn lda(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_parameterized_cpu_state, CpuState as CS};

    #[test]
    fn lda_immediate() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn lda_zero_page() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn lda_zero_page_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn lda_absolute() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn lda_absolute_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn lda_absolute_y() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn lda_indexed_indirect() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn lda_indirect_indexed() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }
}
