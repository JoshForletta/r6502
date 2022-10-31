use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const STY_ZERO_PAGE: Instruction = Instruction {
    opcode: 0x84,
    mnemonic: "STY",
    am: addressing_mode::ZERO_PAGE,
    call: sty,
};

pub const STY_ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x94,
    mnemonic: "STY",
    am: addressing_mode::ZERO_PAGE_X,
    call: sty,
};

pub const STY_ABSOLUTE: Instruction = Instruction {
    opcode: 0x8C,
    mnemonic: "STY",
    am: addressing_mode::ABSOLUTE,
    call: sty,
};

pub fn sty(cpu: &mut R6502, am: AddressingMode) -> Result<(), Box<dyn Error>> {
    (am.call)(cpu)?;
    cpu.bus.write(cpu.target_address, cpu.y)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};

    #[test]
    fn sty() {
        let est = EmulationStateTest {
            instructions: &[0x84, 0x02],
            initial_cpu_state: CpuState {
                y: Some(0x69),
                ..Default::default()
            },
            mem_tests: &[(0x0002, 0x69)],
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
