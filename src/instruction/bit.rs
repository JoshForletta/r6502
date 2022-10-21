use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const BIT_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x24,
    mnemonic: "BIT",
    am: addressing_mode::ZERO_PAGE,
    call: bit,
};

pub const BIT_ABSOLUTE: Instruction = Instruction {
    opcode: 0x2C,
    mnemonic: "BIT",
    am: addressing_mode::ABSOLUTE,
    call: bit,
};

pub fn bit(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let data = *(am.call)(cpu)?;

    cpu.ps.set(PS::Z, (cpu.a ^ data) == 0);
    cpu.ps.set(PS::V, (data & 0x40) != 0);
    cpu.ps.set(PS::N, (data & 0x80) != 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn bit_overflow_flag() {
        let est = EmulationStateTest {
            instructions: &[0x24, 0x02, 0x40],
            test_cpu_state: CpuState {
                ps: Some(PS::V),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn bit_negative_flag() {
        let est = EmulationStateTest {
            instructions: &[0x24, 0x02, 0x80],
            test_cpu_state: CpuState {
                ps: Some(PS::N),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }

    #[test]
    fn bit_zero_page() {
        let est_true = EmulationStateTest {
            instructions: &[0x24, 0x02, 0x0F],
            initial_cpu_state: CpuState {
                a: Some(0x0F),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::Z),
                ..Default::default()
            },
            ..Default::default()
        };

        let est_false = EmulationStateTest {
            instructions: &[0x24, 0x02, 0x0F],
            initial_cpu_state: CpuState {
                a: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::empty()),
                ..Default::default()
            },
            ..Default::default()
        };

        println!("BIT-zpg true:");
        test_emulation_state(&est_true);

        println!("BIT-zpg false:");
        test_emulation_state(&est_false);
    }

    #[test]
    fn bit_absolute() {
        let est_true = EmulationStateTest {
            instructions: &[0x2C, 0x03, 0x00, 0x0F],
            initial_cpu_state: CpuState {
                a: Some(0x0F),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::Z),
                ..Default::default()
            },
            ..Default::default()
        };

        let est_false = EmulationStateTest {
            instructions: &[0x2C, 0x03, 0x00, 0x0F],
            initial_cpu_state: CpuState {
                a: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::empty()),
                ..Default::default()
            },
            ..Default::default()
        };

        println!("BIT-abs true:");
        test_emulation_state(&est_true);

        println!("BIT-abs false:");
        test_emulation_state(&est_false);
    }
}
