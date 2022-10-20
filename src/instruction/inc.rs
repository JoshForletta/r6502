use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
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

pub fn inc(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_parameterized_cpu_state, CpuState as CS};

    #[test]
    fn inc_zero_page() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn inc_zero_page_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn inc_absolute() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn inc_absolute_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }
}
