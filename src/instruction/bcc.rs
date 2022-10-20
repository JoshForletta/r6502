use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const BCC_RELATIVE: Instruction = Instruction {
    opcode: 0x90,
    mnemonic: "BCC",
    am: addressing_mode::RELATIVE,
    call: bcc,
};

pub fn bcc(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_parameterized_cpu_state, CpuState as CS};

    #[test]
    fn bcc_relative() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }
}
