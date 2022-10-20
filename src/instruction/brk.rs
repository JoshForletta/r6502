use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const BRK_IMPLIED: Instruction = Instruction {
    opcode: 0x00,
    mnemonic: "BRK",
    am: addressing_mode::IMPLIED,
    call: brk,
};

pub fn brk(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_parameterized_cpu_state, CpuState as CS};

    #[test]
    fn brk_implied() {
        let tests: &[(&[u8], CS, usize)] = &[];

        test_parameterized_cpu_state(tests);
    }
}
