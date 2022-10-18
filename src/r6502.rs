use std::fmt::Debug;

use bitflags::bitflags;
use byteorder::{ByteOrder, LittleEndian};

use crate::{bus::Bus, device::Device};

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
pub struct R6502<B>
where
    B: Bus<u16, u8>,
{
    ps: PS,  // Processor Status Register
    a: u8,   // Accumulator Register
    x: u8,   // X Index Regester
    y: u8,   // Y Index Register
    pc: u16, // Program Counter
    sp: u8,  // Stack Pointer
    bus: B,
}

impl<B> Debug for R6502<B>
where
    B: Bus<u16, u8> + Debug,
{
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

impl<B> R6502<B>
where
    B: Bus<u16, u8>,
{
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

    pub fn mount_device(&mut self, device: Box<dyn Device<u16, u8>>) {
        self.bus.mount_device(device);
    }

    pub fn clock() {}
}
