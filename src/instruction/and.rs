use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const AND_IMMEDIATE: Instruction = Instruction {
    opcode: 0x29,
    mnemonic: "AND",
    am: addressing_mode::IMMEDIATE,
    call: and,
};

pub const AND_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x25,
    mnemonic: "AND",
    am: addressing_mode::ZERO_PAGE,
    call: and,
};

pub const AND_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x35,
    mnemonic: "AND",
    am: addressing_mode::ZERO_PAGE_X,
    call: and,
};

pub const AND_ABSOLUTE: Instruction = Instruction {
    opcode: 0x2D,
    mnemonic: "AND",
    am: addressing_mode::ABSOLUTE,
    call: and,
};

pub const AND_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x3D,
    mnemonic: "AND",
    am: addressing_mode::ABSOLUTE_X,
    call: and,
};

pub const AND_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0x39,
    mnemonic: "AND",
    am: addressing_mode::ABSOLUTE_Y,
    call: and,
};

pub const AND_INDEXED_INDIRECT: Instruction = Instruction {
    opcode: 0x21,
    mnemonic: "AND",
    am: addressing_mode::INDEXED_INDIRECT,
    call: and,
};

pub const AND_INDIRECT_INDEXED: Instruction = Instruction {
    opcode: 0x31,
    mnemonic: "AND",
    am: addressing_mode::INDIRECT_INDEXED,
    call: and,
};

pub fn and(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_parameterized_cpu_state, CpuState as CS};

    #[test]
    fn and_immediate() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn and_zero_page() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn and_zero_page_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn and_absolute() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn and_absolute_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn and_absolute_y() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn and_indexed_indirect() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn and_indirect_indexed() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }
}
