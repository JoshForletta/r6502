use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const EOR_IMMEDIATE: Instruction = Instruction {
    opcode: 0x49,
    mnemonic: "EOR",
    am: addressing_mode::IMMEDIATE,
    call: eor,
};

pub const EOR_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x45,
    mnemonic: "EOR",
    am: addressing_mode::ZERO_PAGE,
    call: eor,
};

pub const EOR_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x55,
    mnemonic: "EOR",
    am: addressing_mode::ZERO_PAGE_X,
    call: eor,
};

pub const EOR_ABSOLUTE: Instruction = Instruction {
    opcode: 0x4D,
    mnemonic: "EOR",
    am: addressing_mode::ABSOLUTE,
    call: eor,
};

pub const EOR_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x5D,
    mnemonic: "EOR",
    am: addressing_mode::ABSOLUTE_X,
    call: eor,
};

pub const EOR_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0x59,
    mnemonic: "EOR",
    am: addressing_mode::ABSOLUTE_Y,
    call: eor,
};

pub const EOR_INDEXED_INDIRECT: Instruction = Instruction {
    opcode: 0x41,
    mnemonic: "EOR",
    am: addressing_mode::INDEXED_INDIRECT,
    call: eor,
};

pub const EOR_INDIRECT_INDEXED: Instruction = Instruction {
    opcode: 0x51,
    mnemonic: "EOR",
    am: addressing_mode::INDIRECT_INDEXED,
    call: eor,
};

pub fn eor(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let data = *(am.call)(cpu)?;

    cpu.a = cpu.a ^ data;

    cpu.ps.set(PS::Z, cpu.a == 0);
    cpu.ps.set(PS::N, (cpu.a & 0x80) != 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn eor_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0x49, 0xF0],
            initial_cpu_state: CpuState {
                a: Some(0xF0),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::Z),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn eor_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0x49, 0x80],
            initial_cpu_state: CpuState {
                a: Some(0x0F),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::N),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn eor() {
        let est = EmulationStateTest {
            instructions: &[0x49, 0x80],
            initial_cpu_state: CpuState {
                a: Some(0x0F),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x8F),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
