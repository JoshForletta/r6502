use std::{error::Error, num::Wrapping};

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const BEQ_RELATIVE: Instruction = Instruction {
    opcode: 0xF0,
    mnemonic: "BEQ",
    am: addressing_mode::RELATIVE,
    call: beq,
};

pub fn beq(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let offset = *(am.call)(cpu)? as i8;

    if cpu.ps.contains(PS::Z) {
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
    fn beq_zero_clear() {
        let est = EmulationStateTest {
            instructions: &[0xF0, 0xFF],
            test_cpu_state: CpuState {
                pc: Some(0x02),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn beq_positive_offset() {
        let est = EmulationStateTest {
            instructions: &[0xF0, 0x01],
            initial_cpu_state: CpuState {
                ps: Some(PS::Z),
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
    fn beq_negative_offset() {
        let est = EmulationStateTest {
            instructions: &[0xF0, 0xFF],
            initial_cpu_state: CpuState {
                ps: Some(PS::Z),
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
