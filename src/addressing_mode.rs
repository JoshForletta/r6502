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
    let addr = match cpu.read_word() {
        Ok(addr) => addr,
        Err(e) => return Err(e.into()),
    };

    match cpu.bus.read_mut(addr) {
        Ok(t) => Ok(t),
        Err(e) => Err(e.into()),
    }
}

pub const ABSOLUTE_X: AddressingMode = AddressingMode {
    mnemonic: "abs,x",
    call: absolute_x,
};

pub fn absolute_x(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let mut addr = match cpu.read_word() {
        Ok(addr) => addr,
        Err(e) => return Err(e.into()),
    };

    let page = addr & 0xff00;

    addr += Into::<u16>::into(cpu.x);

    cpu.ps.set(PS::P, page == (addr & 0xff00));

    match cpu.bus.read_mut(addr) {
        Ok(t) => Ok(t),
        Err(e) => Err(e.into()),
    }
}

pub const ABSOLUTE_Y: AddressingMode = AddressingMode {
    mnemonic: "abs,y",
    call: absolute_y,
};

pub fn absolute_y(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let mut addr = match cpu.read_word() {
        Ok(addr) => addr,
        Err(e) => return Err(e.into()),
    };

    let page = addr & 0xff00;

    addr += Into::<u16>::into(cpu.y);

    cpu.ps.set(PS::P, page == (addr & 0xff00));

    match cpu.bus.read_mut(addr) {
        Ok(t) => Ok(t),
        Err(e) => Err(e.into()),
    }
}

pub const IMMEDIATE: AddressingMode = AddressingMode {
    mnemonic: "#",
    call: immediate,
};

pub fn immediate(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    cpu.read_mut().or_else(|e| Err(e.into()))
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
    Ok(&mut cpu.null_pointer)
}

pub const INDEXED_INDIRECT: AddressingMode = AddressingMode {
    mnemonic: "x,ind",
    call: indexed_indirect,
};

pub fn indexed_indirect(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let mut addr = match cpu.read() {
        Ok(addr) => addr,
        Err(e) => return Err(e.into()),
    };

    addr = (Wrapping(addr) + Wrapping(cpu.x)).0;

    cpu.bus.read_mut(addr.into()).or_else(|e| Err(e.into()))
}

pub const INDIRECT_INDEXED: AddressingMode = AddressingMode {
    mnemonic: "ind,y",
    call: indirect_indexed,
};

pub fn indirect_indexed(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let zpg_addr = match cpu.read() {
        Ok(addr) => addr,
        Err(e) => return Err(e.into()),
    };

    let mut addr = match cpu.bus.read_word(zpg_addr.into()) {
        Ok(addr) => addr,
        Err(e) => return Err(e.into()),
    };

    let page = addr & 0xff00;

    addr = (Wrapping(addr) + Wrapping(cpu.y.into())).0;

    cpu.ps.set(PS::P, page == (addr & 0xff00));

    cpu.bus.read_mut(addr.into()).or_else(|e| Err(e.into()))
}

pub const RELATIVE: AddressingMode = AddressingMode {
    mnemonic: "rel",
    call: relative,
};

pub fn relative(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    cpu.read_mut().or_else(|e| Err(e.into()))
}

pub const ZERO_PAGE: AddressingMode = AddressingMode {
    mnemonic: "zpg",
    call: zero_page,
};

pub fn zero_page(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let addr = match cpu.read() {
        Ok(addr) => addr,
        Err(e) => return Err(e.into()),
    };

    cpu.bus.read_mut(addr.into()).or_else(|e| Err(e.into()))
}

pub const ZERO_PAGE_X: AddressingMode = AddressingMode {
    mnemonic: "zpg,x",
    call: zero_page_x,
};

pub fn zero_page_x(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let mut addr = match cpu.read() {
        Ok(addr) => addr,
        Err(e) => return Err(e.into()),
    };

    addr = (Wrapping(addr) + Wrapping(cpu.x)).0;

    cpu.bus.read_mut(addr.into()).or_else(|e| Err(e.into()))
}

pub const ZERO_PAGE_Y: AddressingMode = AddressingMode {
    mnemonic: "zpg,y",
    call: zero_page_y,
};

pub fn zero_page_y(cpu: &mut R6502) -> Result<&mut u8, Box<dyn Error>> {
    let mut addr = match cpu.read() {
        Ok(addr) => addr,
        Err(e) => return Err(e.into()),
    };

    addr = (Wrapping(addr) + Wrapping(cpu.y)).0;

    cpu.bus.read_mut(addr.into()).or_else(|e| Err(e.into()))
}
