use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use crate::{device::Device, error::BusError};

pub trait Bus
where
    Self: Debug + Default,
{
    fn read(&self, addr: u16) -> Result<u8, BusError>;
    fn write(&mut self, addr: u16, data: u8) -> Result<(), BusError>;
    fn mount_device(&mut self, device: Box<dyn Device>);
}

#[derive(Debug, Default)]
pub struct DebugBus {
    address: Arc<Mutex<u16>>,
    data: Arc<Mutex<u8>>,
    devices: Vec<Box<dyn Device>>,
}

impl Bus for DebugBus {
    fn read(&self, addr: u16) -> Result<u8, BusError> {
        match self.address.lock() {
            Ok(mut m) => *m = addr,
            Err(_) => (),
        };

        for d in &self.devices {
            if d.contains(addr) {
                return Ok(d[addr]);
            }
        }

        Err(BusError::NoDevice(addr))
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<(), BusError> {
        let mut no_device = true;

        match self.address.lock() {
            Ok(mut m) => *m = addr,
            Err(_) => (),
        };

        match self.data.lock() {
            Ok(mut d) => *d = data,
            Err(_) => (),
        };

        for d in &mut self.devices {
            if d.contains(addr) {
                no_device = false;
                d[addr] = data;
            }
        }

        if no_device {
            Err(BusError::NoDevice(addr))
        } else {
            Ok(())
        }
    }

    fn mount_device(&mut self, device: Box<dyn Device>) {
        self.devices.push(device);
    }
}

impl DebugBus {
    pub fn new() -> Self {
        Default::default()
    }
}
