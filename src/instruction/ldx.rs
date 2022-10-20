use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const LDX_IMMEDIATE: Instruction = Instruction {
    opcode: 0xA2,
    mnemonic: "LDX",
    am: addressing_mode::IMMEDIATE,
    call: ldx,
};

pub const LDX_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xA6,
    mnemonic: "LDX",
    am: addressing_mode::ZERO_PAGE,
    call: ldx,
};

pub const LDX_ZERO_PAGE_Y: Instruction = Instruction {
    opcode: 0xB6,
    mnemonic: "LDX",
    am: addressing_mode::ZERO_PAGE_Y,
    call: ldx,
};

pub const LDX_ABSOLUTE: Instruction = Instruction {
    opcode: 0xAE,
    mnemonic: "LDX",
    am: addressing_mode::ABSOLUTE,
    call: ldx,
};

pub const LDX_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0xBE,
    mnemonic: "LDX",
    am: addressing_mode::ABSOLUTE_Y,
    call: ldx,
};

pub fn ldx(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_parameterized_cpu_state, CpuState as CS};

    #[test]
    fn ldx_immediate() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn ldx_zero_page() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn ldx_zero_page_y() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn ldx_absolute() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn ldx_absolute_y() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }
}
