use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const STY_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x84,
    mnemonic: "STY",
    am: addressing_mode::ZERO_PAGE,
    call: sty,
};

pub const STY_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x94,
    mnemonic: "STY",
    am: addressing_mode::ZERO_PAGE_X,
    call: sty,
};

pub const STY_ABSOLUTE: Instruction = Instruction {
    opcode: 0x8C,
    mnemonic: "STY",
    am: addressing_mode::ABSOLUTE,
    call: sty,
};

pub fn sty(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_parameterized_cpu_state, CpuState as CS};

    #[test]
    fn sty_zero_page() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn sty_zero_page_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn sty_absolute() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }
}
