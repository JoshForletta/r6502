use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const CMP_IMMEDIATE: Instruction = Instruction {
    opcode: 0xC9,
    mnemonic: "CMP",
    am: addressing_mode::IMMEDIATE,
    call: cmp,
};

pub const CMP_ZERO_PAGE: Instruction = Instruction {
    opcode: 0xC5,
    mnemonic: "CMP",
    am: addressing_mode::ZERO_PAGE,
    call: cmp,
};

pub const CMP_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0xD5,
    mnemonic: "CMP",
    am: addressing_mode::ZERO_PAGE_X,
    call: cmp,
};

pub const CMP_ABSOLUTE: Instruction = Instruction {
    opcode: 0xCD,
    mnemonic: "CMP",
    am: addressing_mode::ABSOLUTE,
    call: cmp,
};

pub const CMP_ABSOLUTE_X: Instruction = Instruction {
    opcode: 0xDD,
    mnemonic: "CMP",
    am: addressing_mode::ABSOLUTE_X,
    call: cmp,
};

pub const CMP_ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0xD9,
    mnemonic: "CMP",
    am: addressing_mode::ABSOLUTE_Y,
    call: cmp,
};

pub const CMP_INDEXED_INDIRECT: Instruction = Instruction {
    opcode: 0xC1,
    mnemonic: "CMP",
    am: addressing_mode::INDEXED_INDIRECT,
    call: cmp,
};

pub const CMP_INDIRECT_INDEXED: Instruction = Instruction {
    opcode: 0xD1,
    mnemonic: "CMP",
    am: addressing_mode::INDIRECT_INDEXED,
    call: cmp,
};

pub fn cmp(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    let data = *(am.call)(cpu)?;

    cpu.ps.set(PS::C, cpu.a >= data);
    cpu.ps.set(PS::Z, cpu.a == data);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn cmp_immediate() {
        let est = EmulationStateTest {
            instructions: &[0xC9, 0x69],
            initial_cpu_state: CpuState {
                a: Some(0x69),
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

    // #[test]
    // fn cmp_zero_page() {}
    //
    // #[test]
    // fn cmp_zero_page_x() {}
    //
    // #[test]
    // fn cmp_absolute() {}
    //
    // #[test]
    // fn cmp_absolute_x() {}
    //
    // #[test]
    // fn cmp_absolute_y() {}
    //
    // #[test]
    // fn cmp_indexed_indirect() {}
    //
    // #[test]
    // fn cmp_indirect_indexed() {}
}
