use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const SBC_IMMEDIATE: Instruction = Instruction {
    opcode: 0xE9,
    mnemonic: "SBC",
    am: addressing_mode::IMMEDIATE,
    call: sbc,
};

pub const SBC_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xE5,
    mnemonic: "SBC",
    am: addressing_mode::ZERO_PAGE,
    call: sbc,
};

pub const SBC_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0xF5,
    mnemonic: "SBC",
    am: addressing_mode::ZERO_PAGE_X,
    call: sbc,
};

pub const SBC_ABSOLUTE: Instruction = Instruction {
    opcode: 0xED,
    mnemonic: "SBC",
    am: addressing_mode::ABSOLUTE,
    call: sbc,
};

pub const SBC_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0xFD,
    mnemonic: "SBC",
    am: addressing_mode::ABSOLUTE_X,
    call: sbc,
};

pub const SBC_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0xF9,
    mnemonic: "SBC",
    am: addressing_mode::ABSOLUTE_Y,
    call: sbc,
};

pub const SBC_INDEXED_INDIRECT: Instruction = Instruction {
    opcode: 0xE1,
    mnemonic: "SBC",
    am: addressing_mode::INDEXED_INDIRECT,
    call: sbc,
};

pub const SBC_INDIRECT_INDEXED: Instruction = Instruction {
    opcode: 0xF1,
    mnemonic: "SBC",
    am: addressing_mode::INDIRECT_INDEXED,
    call: sbc,
};

pub fn sbc(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let data = (*(am.call)(cpu)? as u16) ^ 0x00FF;
    let carry = (!cpu.ps.contains(PS::C)) as u16;
    let accumulator = cpu.a as u16;

    let sum = accumulator + data + carry;

    cpu.ps.set(
        PS::V,
        (!(accumulator ^ data) & (accumulator ^ sum) & 0x0080) != 0,
    );

    let (carry, sum) = ((sum >> 8) as u8, sum as u8);

    cpu.ps.set(PS::C, carry != 0);
    cpu.ps.set(PS::Z, sum == 0);
    cpu.ps.set(PS::N, (sum & 0x80) != 0);

    cpu.a = sum;

    if cpu.ps.contains(PS::P) {
        cpu.extra_cycles += 1;
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
    fn sbc_carry_flag() {
        let est = EmulationStateTest {
            instructions: &[0xE9, 0xFD],
            initial_cpu_state: CpuState {
                a: Some(0xFF),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::C),
                ..Default::default()
            },
            clock_cycles: 2,
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn sbc_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0xE9, 0x01],
            initial_cpu_state: CpuState {
                a: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::Z | PS::C),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn sbc_overflow_flag() {
        let est = EmulationStateTest {
            instructions: &[0xE9, 0xFE],
            initial_cpu_state: CpuState {
                a: Some(0x7F),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::V | PS::N),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn sbc_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0xE9, 0x7F],
            initial_cpu_state: CpuState {
                a: Some(0x05),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::N),
                ..Default::default()
            },
            clock_cycles: 2,
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn sbc() {
        let est = EmulationStateTest {
            instructions: &[0xE9, 0x01],
            initial_cpu_state: CpuState {
                a: Some(0x02),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0x01),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
