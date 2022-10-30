use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const SEC_IMPLIED: Instruction = Instruction {
    opcode: 0x38,
    mnemonic: "SEC",
    am: addressing_mode::IMPLIED,
    call: sec,
};

pub fn sec(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.ps.set(PS::C, true);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn sec() {
        let est = EmulationStateTest {
            instructions: &[0x38],
            test_cpu_state: CpuState {
                ps: Some(PS::C),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
