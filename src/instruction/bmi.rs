use std::{error::Error, num::Wrapping};

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const BMI_RELATIVE: Instruction = Instruction {
    opcode: 0x30,
    mnemonic: "BMI",
    am: addressing_mode::RELATIVE,
    call: bmi,
};

pub fn bmi(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let offset = *(am.call)(cpu)? as i8;

    if cpu.ps.contains(PS::N) {
        cpu.pc = (Wrapping(cpu.pc as i16) + Wrapping(Into::<i16>::into(offset))).0 as u16;
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
    fn bmi_negative_clear() {
        let est = EmulationStateTest {
            instructions: &[0x30, 0xFF],
            test_cpu_state: CpuState {
                pc: Some(0x02),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn bmi_positive_offset() {
        let est = EmulationStateTest {
            instructions: &[0x30, 0x01],
            initial_cpu_state: CpuState {
                ps: Some(PS::N),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                pc: Some(0x03),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn bmi_negative_offset() {
        let est = EmulationStateTest {
            instructions: &[0x30, 0xFF],
            initial_cpu_state: CpuState {
                ps: Some(PS::N),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                pc: Some(0x01),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
