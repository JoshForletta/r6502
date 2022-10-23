use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const CLD_IMPLIED: Instruction = Instruction {
    opcode: 0xD8,
    mnemonic: "CLD",
    am: addressing_mode::IMPLIED,
    call: cld,
};

pub fn cld(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.ps.remove(PS::D);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn cld() {
        let est = EmulationStateTest {
            instructions: &[0xD8],
            initial_cpu_state: CpuState {
                ps: Some(PS::D),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::empty()),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
