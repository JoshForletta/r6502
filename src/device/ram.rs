use std::{fmt::Debug, ops::Range};

use crate::{device::Device, error::DeviceError};

#[derive(Default)]
pub struct Ram {
    address_range: Range<u16>,
    m: Vec<u8>,
}

impl Debug for Ram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ram")
            .field(
                "address_range",
                &format_args!(
                    "${:X}..${:X}",
                    self.address_range.start, self.address_range.end
                ),
            )
            .finish()
    }
}

impl Ram {
    pub fn with_address_range(start: u16, end: u16) -> Self {
        Self {
            address_range: Range { start, end },
            m: vec![u8::from(0); (end - start).into()],
        }
    }
}

impl Device for Ram {
    fn address_range(&self) -> &std::ops::Range<u16> {
        &self.address_range
    }

    fn get(&self, addr: u16) -> Result<&u8, DeviceError> {
        match self
            .m
            .get(Into::<usize>::into(addr - self.address_range.start))
        {
            Some(d) => Ok(d),
            None => Err(DeviceError::FailedGet("ram", addr, None)),
        }
    }

    fn get_mut(&mut self, addr: u16) -> Result<&mut u8, DeviceError> {
        match self
            .m
            .get_mut(Into::<usize>::into(addr - self.address_range.start))
        {
            Some(d) => Ok(d),
            None => Err(DeviceError::FailedGet("ram", addr, None)),
        }
    }
}
