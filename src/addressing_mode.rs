use std::{error::Error, num::Wrapping};

use crate::{r6502::PS, R6502};

pub type AmFn = fn(&mut R6502) -> Result<&mut u8, Box<dyn Error>>;

#[derive(Clone, Copy)]
pub struct AddressingMode {
    pub mnemonic: &'static str,
    pub call: AmFn,
}

pub const ACCUMULATOR: AddressingMode = AddressingMode {
    mnemonic: "A",
    call: accumulator,
};

pub fn accumulator(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    Ok(&mut cpu.a)
}

pub const ABSOLUTE: AddressingMode = AddressingMode {
    mnemonic: "abs",
    call: absolute,
};

pub fn absolute(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let addr = cpu.read_word()?;

    cpu.target_address = addr;

    Ok(cpu.bus.read_mut(addr)?)
}

pub const ABSOLUTE_X: AddressingMode = AddressingMode {
    mnemonic: "abs,x",
    call: absolute_x,
};

pub fn absolute_x(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let mut addr = cpu.read_word()?;

    let page = addr & 0xff00;

    addr += cpu.x as u16;

    cpu.target_address = addr;

    cpu.ps.set(PS::P, page == (addr & 0xff00));

    Ok(cpu.bus.read_mut(addr)?)
}

pub const ABSOLUTE_Y: AddressingMode = AddressingMode {
    mnemonic: "abs,y",
    call: absolute_y,
};

pub fn absolute_y(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let mut addr = cpu.read_word()?;

    let page = addr & 0xff00;

    addr += cpu.y as u16;

    cpu.target_address = addr;

    cpu.ps.set(PS::P, page == (addr & 0xff00));

    Ok(cpu.bus.read_mut(addr)?)
}

pub const IMMEDIATE: AddressingMode = AddressingMode {
    mnemonic: "#",
    call: immediate,
};

pub fn immediate(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    cpu.target_address = cpu.pc;
    Ok(cpu.read_mut()?)
}

pub const IMPLIED: AddressingMode = AddressingMode {
    mnemonic: "impl",
    call: implied,
};

pub fn implied(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    Ok(&mut cpu.null_pointer)
}

pub const INDIRECT: AddressingMode = AddressingMode {
    mnemonic: "ind",
    call: indirect,
};

pub fn indirect(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let addr = cpu.read_word()?;
    let addr = cpu.bus.read_word(addr)?;

    cpu.target_address = addr;

    Ok(cpu.bus.read_mut(addr)?)
}

pub const INDEXED_INDIRECT: AddressingMode = AddressingMode {
    mnemonic: "x,ind",
    call: indexed_indirect,
};

pub fn indexed_indirect(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let mut addr = cpu.read()?;

    addr = (Wrapping(addr) + Wrapping(cpu.x)).0;

    let addr = cpu.bus.read_word(addr as u16)?;

    cpu.target_address = addr;

    Ok(cpu.bus.read_mut(addr)?)
}

pub const INDIRECT_INDEXED: AddressingMode = AddressingMode {
    mnemonic: "ind,y",
    call: indirect_indexed,
};

pub fn indirect_indexed(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let addr = cpu.read()?;

    let mut addr = cpu.bus.read_word(addr as u16)?;

    let page = addr & 0xff00;

    addr = (Wrapping(addr) + Wrapping(cpu.y.into())).0;

    cpu.target_address = addr;

    cpu.ps.set(PS::P, page == (addr & 0xff00));

    Ok(cpu.bus.read_mut(addr)?)
}

pub const RELATIVE: AddressingMode = AddressingMode {
    mnemonic: "rel",
    call: relative,
};

pub fn relative(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    cpu.target_address = cpu.pc;
    Ok(cpu.read_mut()?)
}

pub const ZERO_PAGE: AddressingMode = AddressingMode {
    mnemonic: "zpg",
    call: zero_page,
};

pub fn zero_page(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let addr = cpu.read()? as u16;

    cpu.target_address = addr;

    Ok(cpu.bus.read_mut(addr)?)
}

pub const ZERO_PAGE_X: AddressingMode = AddressingMode {
    mnemonic: "zpg,x",
    call: zero_page_x,
};

pub fn zero_page_x(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let mut addr = cpu.read()?;

    addr = (Wrapping(addr) + Wrapping(cpu.x)).0;

    cpu.target_address = addr as u16;

    Ok(cpu.bus.read_mut(addr as u16)?)
}

pub const ZERO_PAGE_Y: AddressingMode = AddressingMode {
    mnemonic: "zpg,y",
    call: zero_page_y,
};

pub fn zero_page_y(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let mut addr = cpu.read()?;

    addr = (Wrapping(addr) + Wrapping(cpu.y)).0;

    cpu.target_address = addr as u16;

    Ok(cpu.bus.read_mut(addr as u16)?)
}
