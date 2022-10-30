use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    R6502,
};

pub const TXS_IMPLIED: Instruction = Instruction {
    opcode: 0x9A,
    mnemonic: "TXS",
    am: addressing_mode::IMPLIED,
    call: txs,
};

pub fn txs(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.sp = cpu.x;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_emulation_state, CpuState, EmulationStateTest};

    #[test]
    fn txs() {
        let est = EmulationStateTest {
            instructions: &[0x9A],
            initial_cpu_state: CpuState {
                x: Some(0x69),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                sp: Some(0x69),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
