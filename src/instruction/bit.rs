use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const BIT_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x24,
    mnemonic: "BIT",
    am: addressing_mode::ZERO_PAGE,
    call: bit,
};

pub const BIT_ABSOLUTE: Instruction = Instruction {
    opcode: 0x2C,
    mnemonic: "BIT",
    am: addressing_mode::ABSOLUTE,
    call: bit,
};

pub fn bit(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_parameterized_cpu_state, CpuState as CS};

    #[test]
    fn bit_zero_page() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn bit_absolute() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }
}
