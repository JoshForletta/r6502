use std::{error::Error, fmt::Display};

use crate::instruction::Instruction;

#[derive(Debug)]
pub enum EmulationError {
    CpuError(CpuError),
    BusError(BusError),
}

impl Display for EmulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CpuError(e) => write!(f, "CPU error: {}", e),
            Self::BusError(e) => write!(f, "Bus error: {}", e),
        }
    }
}

impl Error for EmulationError {}

#[derive(Debug)]
pub enum CpuError {
    MissingInstuction(u8),
}

impl Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingInstuction(i) => write!(f, "Instruction ${:X} not found", i),
        }
    }
}

impl Error for CpuError {}

#[derive(Debug)]
pub enum BusError {
    NoDevice(u16),
    FailedRead(u16, Option<Box<dyn Error>>),
    FailedWrite(u16, u8, Option<Box<dyn Error>>),
}

impl Display for BusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoDevice(a) => write!(f, "No divice at address ${:X}", a),
            Self::FailedRead(a, Some(e)) => write!(f, "Failed to read at address ${:X}: {}", a, e),
            Self::FailedRead(a, _) => write!(f, "Failed to read at address ${:X}", a),
            Self::FailedWrite(a, d, Some(e)) => {
                write!(f, "Failed to write ${:X} to address ${:X}: {}", a, d, e)
            }
            Self::FailedWrite(a, d, _) => write!(f, "Failed to write ${:X} to address ${:X}", a, d),
        }
    }
}

impl Error for BusError {}

#[derive(Debug)]
pub enum DeviceError {
    FailedGet(&'static str, u16, Option<Box<dyn Error>>),
    FailedSet(&'static str, u16, u8, Option<Box<dyn Error>>),
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedGet(d, a, Some(e)) => write!(
                f,
                "Device: '{d}' failed to get data at address ${:X}: {e}",
                a,
            ),
            Self::FailedGet(d, a, _) => {
                write!(f, "Device: '{d}' failed to get data at address ${:X}", a)
            }
            Self::FailedSet(d, a, data, Some(e)) => write!(
                f,
                "Device: '{d}' failed to set data: ${:X} to address: ${:X}: {e}",
                data, a
            ),
            Self::FailedSet(d, a, data, _) => write!(
                f,
                "Device: '{d}' failed to set data: ${:X} to address: ${:X}",
                data, a
            ),
        }
    }
}

impl Error for DeviceError {}

#[derive(Debug)]
pub struct InstructionError {
    pub instruction: Instruction,
    pub error: Box<dyn Error>,
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Instuction: ${:X}: {}",
            self.instruction.opcode, self.error
        )
    }
}

impl Error for InstructionError {}
