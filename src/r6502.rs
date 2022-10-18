use std::{error::Error, fmt::Debug, num::Wrapping};

use bitflags::bitflags;
use byteorder::{ByteOrder, LittleEndian};

use crate::{
    bus::Bus,
    device::Device,
    error::{BusError, CpuError},
};

pub struct Instruction {}

const INSTRUCTION_SET: &[Instruction] = &[Instruction {}];

// Processor Status Flags
bitflags! {
    #[derive(Default)]
    pub struct PS: u8 {
        const C = 0b00000001; // Carry
        const Z = 0b00000010; // Zero
        const I = 0b00000100; // Interupt disable
        const D = 0b00001000; // Decimal mode
        const B = 0b00010000; // BRK command
        const V = 0b01000000; // Overflow
        const N = 0b10000000; // Negative
    }
}

// Rust 6502
#[derive(Default)]
pub struct R6502 {
    ps: PS,   // Processor Status Register
    a: u8,    // Accumulator Register
    x: u8,    // X Index Regester
    y: u8,    // Y Index Register
    pc: u16,  // Program Counter
    sp: u8,   // Stack Pointer
    bus: Bus, // 16bit address bus and 8bit data bus
    extra_cycles: u8,
}

impl Debug for R6502 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("R6502")
            .field("Processor Status", &self.ps)
            .field("Accumulator", &format_args!("${:X}", self.a))
            .field("X Index", &format_args!("${:X}", self.x))
            .field("Y Index", &format_args!("${:X}", self.y))
            .field("Progam Counter", &format_args!("${:X}", self.pc))
            .field("Stack Pointer", &format_args!("${:X}", self.sp))
            .field("Bus", &self.bus)
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

        self.pc = LittleEndian::read_u16(&[
            self.bus.read(0xfffc).unwrap_or(0x00),
            self.bus.read(0xfffd).unwrap_or(0x00),
        ]);
    }

    pub fn mount_device(&mut self, device: Box<dyn Device>) {
        self.bus.mount_device(device);
    }

    fn read(&mut self) -> Result<u8, BusError> {
        let r = self.bus.read(self.pc);
        self.pc = (Wrapping(self.pc) + Wrapping(1)).0;
        r
    }

    pub fn clock(&mut self) -> Result<(), Box<dyn Error>> {
        if self.extra_cycles == 0 {
            let opcode = self.read()?;
            match INSTRUCTION_SET.get::<usize>(opcode.into()) {
                Some(_) => (), //i(&mut self),
                None => return Err(Box::new(CpuError::MissingInstuction(opcode))),
            };
        } else {
            self.extra_cycles -= 1;
        }

        Ok(())
    }
}
