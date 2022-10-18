use std::{error::Error, fmt::Display};

use crate::core::BitWidth;

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

pub enum InstructionError {}

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
