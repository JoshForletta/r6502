use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const PLP_IMPLIED: Instruction = Instruction {
    opcode: 0x28,
    mnemonic: "PLP",
    am: addressing_mode::IMPLIED,
    call: plp,
};

pub fn plp(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.ps = PS::from_bits(cpu.pull()?).unwrap_or(PS::empty());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn plp() {
        let est = EmulationStateTest {
            instructions: &[0x28],
            initial_mem: &[(0x0100, (PS::C | PS::N).bits())],
            initial_cpu_state: CpuState {
                sp: Some(0x01),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::C | PS::N),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
