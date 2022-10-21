use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const AND_IMMEDIATE: Instruction = Instruction {
    opcode: 0x29,
    mnemonic: "AND",
    am: addressing_mode::IMMEDIATE,
    call: and,
};

pub const AND_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x25,
    mnemonic: "AND",
    am: addressing_mode::ZERO_PAGE,
    call: and,
};

pub const AND_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x35,
    mnemonic: "AND",
    am: addressing_mode::ZERO_PAGE_X,
    call: and,
};

pub const AND_ABSOLUTE: Instruction = Instruction {
    opcode: 0x2D,
    mnemonic: "AND",
    am: addressing_mode::ABSOLUTE,
    call: and,
};

pub const AND_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x3D,
    mnemonic: "AND",
    am: addressing_mode::ABSOLUTE_X,
    call: and,
};

pub const AND_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0x39,
    mnemonic: "AND",
    am: addressing_mode::ABSOLUTE_Y,
    call: and,
};

pub const AND_INDEXED_INDIRECT: Instruction = Instruction {
    opcode: 0x21,
    mnemonic: "AND",
    am: addressing_mode::INDEXED_INDIRECT,
    call: and,
};

pub const AND_INDIRECT_INDEXED: Instruction = Instruction {
    opcode: 0x31,
    mnemonic: "AND",
    am: addressing_mode::INDIRECT_INDEXED,
    call: and,
};

pub fn and(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let data = *(am.call)(cpu)?;

    cpu.a = cpu.a & data;

    cpu.ps.set(PS::Z, cpu.a == 0);
    cpu.ps.set(PS::N, (cpu.a & 0x80) != 0);

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
    fn and_zero_flag() {
        let est = EmulationStateTest {
            instructions: &[0x29, 0x0F],
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
    fn and_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0x29, 0xF0],
            initial_cpu_state: CpuState {
                a: Some(0xF0),
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
    fn and_immediate() {
        let est = EmulationStateTest {
            instructions: &[0x29, 0xF0],
            initial_cpu_state: CpuState {
                a: Some(0xFF),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0xF0),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn and_zero_page() {
        let est = EmulationStateTest {
            instructions: &[0x25, 0x02, 0xF0],
            initial_cpu_state: CpuState {
                a: Some(0xFF),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0xF0),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn and_zero_page_x() {
        let est = EmulationStateTest {
            instructions: &[0x35, 0x01, 0xF0],
            initial_cpu_state: CpuState {
                a: Some(0xFF),
                x: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0xF0),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn and_absolute() {
        let est = EmulationStateTest {
            instructions: &[0x2D, 0x03, 0x00, 0xF0],
            initial_cpu_state: CpuState {
                a: Some(0xFF),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0xF0),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn and_absolute_x() {
        let est = EmulationStateTest {
            instructions: &[0x3D, 0x02, 0x00, 0xF0],
            initial_cpu_state: CpuState {
                a: Some(0xFF),
                x: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0xF0),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn and_absolute_y() {
        let est = EmulationStateTest {
            instructions: &[0x39, 0x02, 0x00, 0xF0],
            initial_cpu_state: CpuState {
                a: Some(0xFF),
                y: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0xF0),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn and_indexed_indirect() {
        let est = EmulationStateTest {
            instructions: &[0x21, 0x01, 0x04, 0x00, 0xF0],
            initial_cpu_state: CpuState {
                a: Some(0xFF),
                x: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0xF0),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn and_indirect_indexed() {
        let est = EmulationStateTest {
            instructions: &[0x31, 0x02, 0x03, 0x00, 0xF0],
            initial_cpu_state: CpuState {
                a: Some(0xFF),
                y: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                a: Some(0xF0),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
