use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
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

pub fn adc(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let d = *(am.call)(cpu)? as u16;
    let c = cpu.ps.contains(PS::C) as u16;
    let a = cpu.a as u16;

    let sum = a + d + c;

    cpu.ps.set(PS::V, (!(a ^ d) & (a ^ sum) & 0x0080) != 0);

    let (carry, sum) = ((sum >> 8) as u8, sum as u8);

    cpu.ps.set(PS::C, carry != 0);
    cpu.ps.set(PS::Z, sum == 0);
    cpu.ps.set(PS::N, (sum & 0x80) != 0);

    cpu.a = sum;

    if cpu.ps.contains(PS::P) {
        cpu.extra_cycles += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn adc_carry_flag() {
        let est = EmulationStateTest {
            instructions: &[0x69, 0x02],
            initial_cpu_state: CpuState {
                a: Some(0xFF),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x01),
                ps: Some(PS::C),
                ..Default::default()
            },
            clock_cycles: 2,
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn adc_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0x69, 0x01],
            initial_cpu_state: CpuState {
                a: Some(0xFF),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x00),
                ps: Some(PS::Z | PS::C),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn adc_overflow_flag() {
        let est = EmulationStateTest {
            instructions: &[0x69, 0x01],
            initial_cpu_state: CpuState {
                a: Some(0x7F),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x80),
                ps: Some(PS::V | PS::N),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn adc_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0x69, 0x80],
            initial_cpu_state: CpuState {
                a: Some(0x05),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x85),
                ps: Some(PS::N),
                ..Default::default()
            },
            clock_cycles: 2,
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn adc_immediate() {
        let est = EmulationStateTest {
            instructions: &[0x69, 0x01],
            initial_cpu_state: CpuState {
                a: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x02),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn adc_zero_page() {
        let est = EmulationStateTest {
            instructions: &[0x65, 0x02, 0x05],
            initial_cpu_state: CpuState {
                a: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x06),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn adc_zero_page_x() {
        let est = EmulationStateTest {
            instructions: &[0x75, 0x02, 0x00, 0x00, 0x05],
            initial_cpu_state: CpuState {
                a: Some(0x01),
                x: Some(0x02),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x06),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn adc_absolute() {
        let est = EmulationStateTest {
            instructions: &[0x6D, 0x03, 0x00, 0x05],
            initial_cpu_state: CpuState {
                a: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x06),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn adc_absolute_x() {
        let est = EmulationStateTest {
            instructions: &[0x7D, 0x03, 0x00, 0x00, 0x05],
            initial_cpu_state: CpuState {
                a: Some(0x01),
                x: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x06),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn adc_absolute_y() {
        let est = EmulationStateTest {
            instructions: &[0x79, 0x03, 0x00, 0x00, 0x05],
            initial_cpu_state: CpuState {
                a: Some(0x01),
                y: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x06),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn adc_indexed_indirect() {
        let est = EmulationStateTest {
            instructions: &[0x61, 0x01, 0x04, 0x00, 0x05],
            initial_cpu_state: CpuState {
                a: Some(0x01),
                x: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x06),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn adc_indirect_indexed() {
        let est = EmulationStateTest {
            instructions: &[0x71, 0x02, 0x03, 0x00, 0x05],
            initial_cpu_state: CpuState {
                a: Some(0x01),
                y: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x06),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
