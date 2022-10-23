use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const CPY_IMMEDIATE: Instruction = Instruction {
    opcode: 0xC0,
    mnemonic: "CPY",
    am: addressing_mode::IMMEDIATE,
    call: cpy,
};

pub const CPY_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xC4,
    mnemonic: "CPY",
    am: addressing_mode::ZERO_PAGE,
    call: cpy,
};

pub const CPY_ABSOLUTE: Instruction = Instruction {
    opcode: 0xCC,
    mnemonic: "CPY",
    am: addressing_mode::ABSOLUTE,
    call: cpy,
};

pub fn cpy(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let data = *(am.call)(cpu)?;

    cpu.ps.set(PS::C, cpu.y >= data);
    cpu.ps.set(PS::Z, cpu.y == data);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn cpy() {
        let est = EmulationStateTest {
            instructions: &[0xC0, 0x69],
            initial_cpu_state: CpuState {
                y: Some(0x69),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::C | PS::Z),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
