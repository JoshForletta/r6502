use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const PHP_IMPLIED: Instruction = Instruction {
    opcode: 0x08,
    mnemonic: "PHP",
    am: addressing_mode::IMPLIED,
    call: php,
};

pub fn php(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.push(cpu.ps.bits())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn php() {
        let est = EmulationStateTest {
            instructions: &[0x08],
            initial_cpu_state: CpuState {
                ps: Some(PS::C | PS::N),
                ..Default::default()
            },
            mem_tests: &[(0x0100, (PS::C | PS::N).bits())],
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
