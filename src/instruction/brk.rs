use std::error::Error;

use crate::{
    addressing_mode::{self, AddressingMode},
    instruction::Instruction,
    r6502::PS,
    R6502,
};

pub const BRK_IMPLIED: Instruction = Instruction {
    opcode: 0x00,
    mnemonic: "BRK",
    am: addressing_mode::IMPLIED,
    call: brk,
};

pub fn brk(cpu: &mut R6502, _am: AddressingMode) -> Result<(), Box<dyn Error>> {
    cpu.push_word(cpu.pc)?;
    cpu.push(cpu.sp)?;

    cpu.pc = cpu.bus.read_word(0xFFFE)?;

    cpu.ps = cpu.ps | PS::B;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        r6502::PS,
        test_utils::{test_emulation_state, CpuState, EmulationStateTest},
    };

    #[test]
    fn brk() {
        let est = EmulationStateTest {
            instructions: &[0x00],
            initial_cpu_state: CpuState {
                ps: Some(PS::Z),
                ..Default::default()
            },
            test_cpu_state: CpuState {
                ps: Some(PS::B | PS::Z),
                pc: Some(0xFF69),
                ..Default::default()
            },
            initial_mem: &[(0xFFFE, 0x69), (0xFFFF, 0xFF)],
            mem_tests: &[(0x0100, 0x01), (0x0101, 0x00), (0x0102, 0x02)],
            ..Default::default()
        };

        test_emulation_state(&est);
    }
}
