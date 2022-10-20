use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const ROR_ACCUMULATOR: Instruction = Instruction {
    opcode: 0x6A,
    mnemonic: "ROR",
    am: addressing_mode::ACCUMULATOR,
    call: ror,
};

pub const ROR_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x66,
    mnemonic: "ROR",
    am: addressing_mode::ZERO_PAGE,
    call: ror,
};

pub const ROR_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x76,
    mnemonic: "ROR",
    am: addressing_mode::ZERO_PAGE_X,
    call: ror,
};

pub const ROR_ABSOLUTE: Instruction = Instruction {
    opcode: 0x6E,
    mnemonic: "ROR",
    am: addressing_mode::ABSOLUTE,
    call: ror,
};

pub const ROR_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x7E,
    mnemonic: "ROR",
    am: addressing_mode::ABSOLUTE_X,
    call: ror,
};

pub fn ror(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_parameterized_cpu_state, CpuState as CS};

    #[test]
    fn ror_accumulator() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn ror_zero_page() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn ror_zero_page_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn ror_absolute() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn ror_absolute_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }
}
