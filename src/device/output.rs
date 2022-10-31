use std::{
    fmt::Debug,
    io::{stdout, Write},
    ops::Range,
};

use crate::{device::Device, error::DeviceError};

#[derive(Debug)]
pub struct Output {
    address_range: Range<u64>,
}

impl Output {
    pub fn with_address(addr: u16) -> Self {
        Self {
            address_range: Range {
                start: addr as u64,
                end: addr as u64 + 1,
            },
        }
    }
}

impl Device for Output {
    fn set(&mut self, _addr: u16, data: u8) -> Result<(), DeviceError> {
        print!("{}", data as char);
        stdout().flush().unwrap();

        Ok(())
    }

    fn address_range(&self) -> &Range<u64> {
        &self.address_range
    }

    fn get(&self, addr: u16) -> Result<&u8, DeviceError> {
        Err(DeviceError::FailedGet("Output", addr, None))
    }

    fn get_mut(&mut self, addr: u16) -> Result<&mut u8, DeviceError> {
        Err(DeviceError::FailedGet("Output", addr, None))
    }
}
