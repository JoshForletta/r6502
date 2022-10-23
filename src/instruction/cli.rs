use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const CLI_IMPLIED: Instruction = Instruction {
    opcode: 0x58,
    mnemonic: "CLI",
    am: addressing_mode::IMPLIED,
    call: cli,
};

pub fn cli(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.ps.remove(PS::I);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn cli() {
        let est = EmulationStateTest {
            instructions: &[0x58],
            initial_cpu_state: CpuState {
                ps: Some(PS::I),
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
