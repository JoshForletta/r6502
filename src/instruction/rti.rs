use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const RTI_IMPLIED: Instruction = Instruction {
    opcode: 0x40,
    mnemonic: "RTI",
    am: addressing_mode::IMPLIED,
    call: rti,
};

pub fn rti(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.ps = PS::from_bits(cpu.pull()?).unwrap_or(PS::empty());
    cpu.pc = cpu.pull_word()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn rti() {
        let est = EmulationStateTest {
            instructions: &[0x40],
            initial_mem: &[
                (0x0100, 0x69),
                (0x0101, 0xFF),
                (0x0102, (PS::C | PS::N).bits()),
            ],
            initial_cpu_state: CpuState {
                sp: Some(0x03),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::C | PS::N),
                pc: Some(0xFF69),
                ..Default::default()
            },
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
