use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const LDX_IMMEDIATE: Instruction = Instruction {
    opcode: 0xA2,
    mnemonic: "LDX",
    am: addressing_mode::IMMEDIATE,
    call: ldx,
};

pub const LDX_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xA6,
    mnemonic: "LDX",
    am: addressing_mode::ZERO_PAGE,
    call: ldx,
};

pub const LDX_ZERO_PAGE_Y: Instruction = Instruction {
    opcode: 0xB6,
    mnemonic: "LDX",
    am: addressing_mode::ZERO_PAGE_Y,
    call: ldx,
};

pub const LDX_ABSOLUTE: Instruction = Instruction {
    opcode: 0xAE,
    mnemonic: "LDX",
    am: addressing_mode::ABSOLUTE,
    call: ldx,
};

pub const LDX_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0xBE,
    mnemonic: "LDX",
    am: addressing_mode::ABSOLUTE_Y,
    call: ldx,
};

pub fn ldx(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.x = *(am.call)(cpu)?;

    cpu.ps.set(PS::Z, cpu.x == 0);
    cpu.ps.set(PS::N, (cpu.x & 0x80) != 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn ldx_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0xA2, 0x00],
            test_cpu_state: CpuState {
                ps: Some(PS::Z),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn ldx_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0xA2, 0x80],
            test_cpu_state: CpuState {
                ps: Some(PS::N),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn ldx() {
        let est = EmulationStateTest {
            instructions: &[0xA2, 0x69],
            test_cpu_state: CpuState {
                x: Some(0x69),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
