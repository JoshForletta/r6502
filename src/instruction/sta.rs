use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const STA_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x85,
    mnemonic: "STA",
    am: addressing_mode::ZERO_PAGE,
    call: sta,
};

pub const STA_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x95,
    mnemonic: "STA",
    am: addressing_mode::ZERO_PAGE_X,
    call: sta,
};

pub const STA_ABSOLUTE: Instruction = Instruction {
    opcode: 0x8D,
    mnemonic: "STA",
    am: addressing_mode::ABSOLUTE,
    call: sta,
};

pub const STA_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x9D,
    mnemonic: "STA",
    am: addressing_mode::ABSOLUTE_X,
    call: sta,
};

pub const STA_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0x99,
    mnemonic: "STA",
    am: addressing_mode::ABSOLUTE_Y,
    call: sta,
};

pub const STA_INDEXED_INDIRECT: Instruction = Instruction {
    opcode: 0x81,
    mnemonic: "STA",
    am: addressing_mode::INDEXED_INDIRECT,
    call: sta,
};

pub const STA_INDIRECT_INDEXED: Instruction = Instruction {
    opcode: 0x91,
    mnemonic: "STA",
    am: addressing_mode::INDIRECT_INDEXED,
    call: sta,
};

pub fn sta(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    (am.call)(cpu)?;
    cpu.bus.write(cpu.target_address, cpu.a)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};

    #[test]
    fn sta() {
        let est = EmulationStateTest {
            instructions: &[0x85, 0x02],
            initial_cpu_state: CpuState {
                a: Some(0x69),
                ..Default::default()
            },
            mem_tests: &[(0x0002, 0x69)],
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
