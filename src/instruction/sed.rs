use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const SED_IMPLIED: Instruction = Instruction {
    opcode: 0xF8,
    mnemonic: "SED",
    am: addressing_mode::IMPLIED,
    call: sed,
};

pub fn sed(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.ps.set(PS::D, true);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn sed() {
        let est = EmulationStateTest {
            instructions: &[0xF8],
            test_cpu_state: CpuState {
                ps: Some(PS::D),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
