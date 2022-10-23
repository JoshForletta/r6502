use std::{error::Error, fmt::Debug, ops::Range};

pub mod ram;
pub use ram::Ram;

use crate::error::DeviceError;

pub trait Device<E = DeviceError>
where
    Self: Debug,
    E: Error,
{
    fn address_range(&self) -> &Range<u64>;

    #[inline]
    fn contains(&self, a: u16) -> bool {
        self.address_range().contains(&(a as u64))
    }

    fn get(&self, addr: u16) -> Result<&u8, E>;
    fn get_mut(&mut self, addr: u16) -> Result<&mut u8, E>;

    fn set(&mut self, addr: u16, data: u8) -> Result<(), E> {
        *self.get_mut(addr)? = data;
        Ok(())
    }
}
