use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use crate::{device::Device, error::BusError};

#[derive(Debug, Default)]
pub struct Bus {
    address: Arc<Mutex<u16>>,
    data: Arc<Mutex<u8>>,
    devices: Vec<Box<dyn Device>>,
}

impl Bus {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn read(&self, addr: u16) -> Result<u8, BusError> {
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

    pub fn write(&mut self, addr: u16, data: u8) -> Result<(), BusError> {
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

    pub fn mount_device(&mut self, device: Box<dyn Device>) {
        self.devices.push(device);
    }
}
