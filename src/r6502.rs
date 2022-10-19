use std::{error::Error, fmt::Debug, num::Wrapping};

use bitflags::bitflags;

use crate::{
    bus::Bus,
    device::Device,
    error::{BusError, CpuError},
    instruction::INSTRUCTION_SET,
};

// Processor Status Flags
bitflags! {
    #[derive(Default)]
    pub struct PS: u8 {
        const C = 0b00000001; // Carry
        const Z = 0b00000010; // Zero
        const I = 0b00000100; // Interupt disable
        const D = 0b00001000; // Decimal mode
        const B = 0b00010000; // BRK command
        const P = 0b00100000; // Unused but will indicate page boundry cross
        const V = 0b01000000; // Overflow
        const N = 0b10000000; // Negative
    }
}

// Rust 6502
#[derive(Default)]
pub struct R6502 {
    pub ps: PS,   // Processor Status Register
    pub a: u8,    // Accumulator Register
    pub x: u8,    // X Index Regester
    pub y: u8,    // Y Index Register
    pub pc: u16,  // Program Counter
    pub sp: u8,   // Stack Pointer
    pub bus: Bus, // 16bit address bus and 8bit data bus
    pub extra_cycles: u8,
    pub null_pointer: u8, // for implied addressing mode
}

impl Debug for R6502 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("R6502")
            .field("Processor Status", &self.ps)
            .field("Accumulator", &format_args!("${:04X}", self.a))
            .field("X Index", &format_args!("${:04X}", self.x))
            .field("Y Index", &format_args!("${:04X}", self.y))
            .field("Progam Counter", &format_args!("${:08X}", self.pc))
            .field("Stack Pointer", &format_args!("${:04X}", self.sp))
            .field("Bus", &self.bus)
            .field("extra_cycles", &self.extra_cycles)
            .finish()
    }
}

impl R6502 {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn reset(&mut self) {
        self.ps = PS::B | PS::I;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0;

        self.pc = 0xfffc;
        self.pc = self.read_word().unwrap_or(0x0000)
    }

    pub fn mount_device(&mut self, device: Box<dyn Device>) {
        self.bus.mount_device(device);
    }

    pub fn read(&mut self) -> Result<u8, BusError> {
        let r = self.bus.read(self.pc);
        self.pc = (Wrapping(self.pc) + Wrapping(1)).0;
        r
    }

    pub fn read_mut(&mut self) -> Result<&mut u8, BusError> {
        let r = self.bus.read_mut(self.pc);
        self.pc = (Wrapping(self.pc) + Wrapping(1)).0;
        r
    }

    pub fn read_word(&mut self) -> Result<u16, BusError> {
        let r = self.bus.read_word(self.pc)?;

        self.pc = (Wrapping(self.pc) + Wrapping(2)).0;

        Ok(r)
    }

    pub fn clock(&mut self) -> Result<(), Box<dyn Error>> {
        self.ps.set(PS::P, false);

        if self.extra_cycles == 0 {
            let opcode = self.read()?;
            match INSTRUCTION_SET.get::<usize>(opcode.into()) {
                Some(i) => i.exec(self)?, //i(&mut self),
                None => return Err(Box::new(CpuError::MissingInstuction(opcode))),
            };
        } else {
            self.extra_cycles -= 1;
        }

        Ok(())
    }
}
