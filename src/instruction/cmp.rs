use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const CMP_IMMEDIATE: Instruction = Instruction {
    opcode: 0xC9,
    mnemonic: "CMP",
    am: addressing_mode::IMMEDIATE,
    call: cmp,
};

pub const CMP_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xC5,
    mnemonic: "CMP",
    am: addressing_mode::ZERO_PAGE,
    call: cmp,
};

pub const CMP_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0xD5,
    mnemonic: "CMP",
    am: addressing_mode::ZERO_PAGE_X,
    call: cmp,
};

pub const CMP_ABSOLUTE: Instruction = Instruction {
    opcode: 0xCD,
    mnemonic: "CMP",
    am: addressing_mode::ABSOLUTE,
    call: cmp,
};

pub const CMP_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0xDD,
    mnemonic: "CMP",
    am: addressing_mode::ABSOLUTE_X,
    call: cmp,
};

pub const CMP_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0xD9,
    mnemonic: "CMP",
    am: addressing_mode::ABSOLUTE_Y,
    call: cmp,
};

pub const CMP_INDEXED_INDIRECT: Instruction = Instruction {
    opcode: 0xC1,
    mnemonic: "CMP",
    am: addressing_mode::INDEXED_INDIRECT,
    call: cmp,
};

pub const CMP_INDIRECT_INDEXED: Instruction = Instruction {
    opcode: 0xD1,
    mnemonic: "CMP",
    am: addressing_mode::INDIRECT_INDEXED,
    call: cmp,
};

pub fn cmp(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_parameterized_cpu_state, CpuState as CS};

    #[test]
    fn cmp_immediate() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn cmp_zero_page() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn cmp_zero_page_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn cmp_absolute() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn cmp_absolute_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn cmp_absolute_y() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn cmp_indexed_indirect() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn cmp_indirect_indexed() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }
}
