use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const JMP_ABSOLUTE: Instruction = Instruction {
    opcode: 0x4C,
    mnemonic: "JMP",
    am: addressing_mode::ABSOLUTE,
    call: jmp,
};

pub const JMP_INDIRECT: Instruction = Instruction {
    opcode: 0x6C,
    mnemonic: "JMP",
    am: addressing_mode::INDIRECT,
    call: jmp,
};

pub fn jmp(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_parameterized_cpu_state, CpuState as CS};

    #[test]
    fn jmp_absolute() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }

    #[test]
    fn jmp_indirect() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }
}
