use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
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

pub fn ora(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};
    //
    // #[test]
    // fn ora_immediate() {}
    //
    // #[test]
    // fn ora_zero_page() {}
    //
    // #[test]
    // fn ora_zero_page_x() {}
    //
    // #[test]
    // fn ora_absolute() {}
    //
    // #[test]
    // fn ora_absolute_x() {}
    //
    // #[test]
    // fn ora_absolute_y() {}
    //
    // #[test]
    // fn ora_indexed_indirect() {}
    //
    // #[test]
    // fn ora_indirect_indexed() {}
}
