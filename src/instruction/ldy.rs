use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
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

pub fn ldy(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_parameterized_cpu_state, CpuState as CS};

    #[test]
    fn ldy_immediate() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn ldy_zero_page() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn ldy_zero_page_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn ldy_absolute() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn ldy_absolute_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }
}
