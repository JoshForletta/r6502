use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const ORA_IMMEDIATE: Instruction = Instruction {
    opcode: 0x09,
    mnemonic: "ORA",
    am: addressing_mode::IMMEDIATE,
    call: ora,
};

pub const ORA_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x05,
    mnemonic: "ORA",
    am: addressing_mode::ZERO_PAGE,
    call: ora,
};

pub const ORA_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x15,
    mnemonic: "ORA",
    am: addressing_mode::ZERO_PAGE_X,
    call: ora,
};

pub const ORA_ABSOLUTE: Instruction = Instruction {
    opcode: 0x0D,
    mnemonic: "ORA",
    am: addressing_mode::ABSOLUTE,
    call: ora,
};

pub const ORA_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x1D,
    mnemonic: "ORA",
    am: addressing_mode::ABSOLUTE_X,
    call: ora,
};

pub const ORA_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0x19,
    mnemonic: "ORA",
    am: addressing_mode::ABSOLUTE_Y,
    call: ora,
};

pub const ORA_INDEXED_INDIRECT: Instruction = Instruction {
    opcode: 0x01,
    mnemonic: "ORA",
    am: addressing_mode::INDEXED_INDIRECT,
    call: ora,
};

pub const ORA_INDIRECT_INDEXED: Instruction = Instruction {
    opcode: 0x11,
    mnemonic: "ORA",
    am: addressing_mode::INDIRECT_INDEXED,
    call: ora,
};

pub fn ora(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let data = *(am.call)(cpu)?;

    cpu.a = cpu.a | data;

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
    fn ora_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0x09, 0x00],
            initial_cpu_state: CpuState {
                a: Some(0x00),
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
    fn ora_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0x09, 0x80],
            initial_cpu_state: CpuState {
                a: Some(0x8F),
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
    fn ora() {
        let est = EmulationStateTest {
            instructions: &[0x09, 0xF0],
            initial_cpu_state: CpuState {
                a: Some(0x0F),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0xFF),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
