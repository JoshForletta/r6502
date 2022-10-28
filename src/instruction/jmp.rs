use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const JMP_ABSOLUTE: Instruction = Instruction {
    opcode: 0x4C,
    mnemonic: "JMP",
    am: addressing_mode::ABSOLUTE,
    call: jmp,
};

pub const JMP_INDIRECT: Instruction = Instruction {
    opcode: 0x6C,
    mnemonic: "JMP",
    am: addressing_mode::INDIRECT,
    call: jmp,
};

pub fn jmp(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    (am.call)(cpu)?;

    cpu.pc = cpu.target_address;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};

    #[test]
    fn jmp_absolute() {
        let est = EmulationStateTest {
            instructions: &[0x4C, 0x69, 0xFF],
            test_cpu_state: CpuState {
                pc: Some(0xFF69),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn jmp_indirect() {
        let est = EmulationStateTest {
            instructions: &[0x6C, 0x03, 0x00, 0x69, 0xFF],
            test_cpu_state: CpuState {
                pc: Some(0xFF69),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
