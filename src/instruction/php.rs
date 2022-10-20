use std::error::Error;

use crate::{
    addressing_mode::{self, AmFn},
    instruction::Instruction,
    R6502,
};

pub const PHP_IMPLIED: Instruction = Instruction {
    opcode: 0x08,
    mnemonic: "PHP",
    am: addressing_mode::IMPLIED,
    call: php,
};

pub fn php(cpu: &mut R6502, am: AmFn) -> Result<(), Box<dyn Error>> {
    let _target = am(cpu)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn php_implied() {}
}
