use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const BMI_RELATIVE: Instruction = Instruction {
    opcode: 0x30,
    mnemonic: "BMI",
    am: addressing_mode::RELATIVE,
    call: bmi,
};

pub fn bmi(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn bmi_relative() {}
}
