use std::{error::Error, num::Wrapping};

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const BCC_RELATIVE: Instruction = Instruction {
    opcode: 0x90,
    mnemonic: "BCC",
    am: addressing_mode::RELATIVE,
    call: bcc,
};

pub fn bcc(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let offset = *(am.call)(cpu)? as i8;

    if !cpu.ps.contains(PS::C) {
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
    fn bcc_carry_set() {
        let est = EmulationStateTest {
            instructions: &[0x90, 0xFF],
            initial_cpu_state: CpuState {
                ps: Some(PS::C),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                pc: Some(0x02),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn bcc_positive_offset() {
        let est = EmulationStateTest {
            instructions: &[0x90, 0x02],
            test_cpu_state: CpuState {
                pc: Some(0x04),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn bcc_negative_offset() {
        let est = EmulationStateTest {
            instructions: &[0x90, 0xFF],
            test_cpu_state: CpuState {
                pc: Some(0x01),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
