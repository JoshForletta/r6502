use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const JSR_ABSOLUTE: Instruction = Instruction {
    opcode: 0x20,
    mnemonic: "JSR",
    am: addressing_mode::ABSOLUTE,
    call: jsr,
};

pub fn jsr(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    (am.call)(cpu)?;

    cpu.push_word(cpu.pc - 1)?;

    cpu.pc = cpu.target_address;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};

    #[test]
    fn jsr() {
        let est = EmulationStateTest {
            instructions: &[0x20, 0x69, 0xFF],
            test_cpu_state: CpuState {
                pc: Some(0xFF69),
                ..Default::default()
            },
            mem_tests: &[(0x0100, 0x02), (0x0101, 0x00)],
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
