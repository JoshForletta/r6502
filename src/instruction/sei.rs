use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const SEI_IMPLIED: Instruction = Instruction {
    opcode: 0x78,
    mnemonic: "SEI",
    am: addressing_mode::IMPLIED,
    call: sei,
};

pub fn sei(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.ps.set(PS::I, true);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn sei() {
        let est = EmulationStateTest {
            instructions: &[0x78],
            test_cpu_state: CpuState {
                ps: Some(PS::I),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
