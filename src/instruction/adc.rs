use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const ADC_IMMEDIATE: Instruction = Instruction {
    opcode: 0x69,
    mnemonic: "ADC",
    am: addressing_mode::IMMEDIATE,
    call: adc,
};

pub const ADC_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x65,
    mnemonic: "ADC",
    am: addressing_mode::ZERO_PAGE,
    call: adc,
};

pub const ADC_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x75,
    mnemonic: "ADC",
    am: addressing_mode::ZERO_PAGE_X,
    call: adc,
};

pub const ADC_ABSOLUTE: Instruction = Instruction {
    opcode: 0x6D,
    mnemonic: "ADC",
    am: addressing_mode::ABSOLUTE,
    call: adc,
};

pub const ADC_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x7D,
    mnemonic: "ADC",
    am: addressing_mode::ABSOLUTE_X,
    call: adc,
};

pub const ADC_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0x79,
    mnemonic: "ADC",
    am: addressing_mode::ABSOLUTE_Y,
    call: adc,
};

pub const ADC_INDEXED_INDIRECT: Instruction = Instruction {
    opcode: 0x61,
    mnemonic: "ADC",
    am: addressing_mode::INDEXED_INDIRECT,
    call: adc,
};

pub const ADC_INDIRECT_INDEXED: Instruction = Instruction {
    opcode: 0x71,
    mnemonic: "ADC",
    am: addressing_mode::INDIRECT_INDEXED,
    call: adc,
};

pub fn adc(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_parameterized_cpu_state, CpuState as CS};

    #[test]
    fn adc_immediate() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn adc_zero_page() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn adc_zero_page_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn adc_absolute() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn adc_absolute_x() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn adc_absolute_y() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn adc_indexed_indirect() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn adc_indirect_indexed() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }
}
