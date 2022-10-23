use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const CPX_IMMEDIATE: Instruction = Instruction {
    opcode: 0xE0,
    mnemonic: "CPX",
    am: addressing_mode::IMMEDIATE,
    call: cpx,
};

pub const CPX_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xE4,
    mnemonic: "CPX",
    am: addressing_mode::ZERO_PAGE,
    call: cpx,
};

pub const CPX_ABSOLUTE: Instruction = Instruction {
    opcode: 0xEC,
    mnemonic: "CPX",
    am: addressing_mode::ABSOLUTE,
    call: cpx,
};

pub fn cpx(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let data = *(am.call)(cpu)?;

    cpu.ps.set(PS::C, cpu.x >= data);
    cpu.ps.set(PS::Z, cpu.x == data);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn cpx() {
        let est = EmulationStateTest {
            instructions: &[0xE0, 0x69],
            initial_cpu_state: CpuState {
                x: Some(0x69),
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
