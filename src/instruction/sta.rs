use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const STA_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x85,
    mnemonic: "STA",
    am: addressing_mode::ZERO_PAGE,
    call: sta,
};

pub const STA_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x95,
    mnemonic: "STA",
    am: addressing_mode::ZERO_PAGE_X,
    call: sta,
};

pub const STA_ABSOLUTE: Instruction = Instruction {
    opcode: 0x8D,
    mnemonic: "STA",
    am: addressing_mode::ABSOLUTE,
    call: sta,
};

pub const STA_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x9D,
    mnemonic: "STA",
    am: addressing_mode::ABSOLUTE_X,
    call: sta,
};

pub const STA_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0x99,
    mnemonic: "STA",
    am: addressing_mode::ABSOLUTE_Y,
    call: sta,
};

pub const STA_INDEXED_INDIRECT: Instruction = Instruction {
    opcode: 0x81,
    mnemonic: "STA",
    am: addressing_mode::INDEXED_INDIRECT,
    call: sta,
};

pub const STA_INDIRECT_INDEXED: Instruction = Instruction {
    opcode: 0x91,
    mnemonic: "STA",
    am: addressing_mode::INDIRECT_INDEXED,
    call: sta,
};

pub fn sta(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_parameterized_cpu_state, CpuState as CS};

    #[test]
    fn sta_zero_page() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn sta_zero_page_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn sta_absolute() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn sta_absolute_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn sta_absolute_y() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn sta_indexed_indirect() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn sta_indirect_indexed() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }
}
