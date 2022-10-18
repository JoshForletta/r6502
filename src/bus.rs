use std::sync::{Arc, Mutex};

use crate::{core::BitWidth, device::Device};

pub enum BusError {
    NoDevice,
}

pub trait Bus<Address, Data>
where
    Self: Default,
    Address: BitWidth,
    Data: BitWidth,
{
    fn read(&self, addr: Address) -> Result<Data, BusError>;
    fn write(&mut self, addr: Address, data: Data) -> Result<(), BusError>;
    fn mount_device(&mut self, device: Box<dyn Device<Address, Data>>);
}

#[derive(Debug, Default)]
pub struct DebugBus<Address, Data>
where
    Address: BitWidth,
    Data: BitWidth,
{
    address: Arc<Mutex<Address>>,
    data: Arc<Mutex<Data>>,
    devices: Vec<Box<dyn Device<Address, Data>>>,
}

impl<Address, Data> Bus<Address, Data> for DebugBus<Address, Data>
where
    Address: BitWidth,
    Data: BitWidth,
{
    fn read(&self, addr: Address) -> Result<Data, BusError> {
        match self.address.lock() {
            Ok(mut m) => *m = addr,
            Err(_) => (),
        };

        for d in &self.devices {
            if d.contains(addr) {
                return Ok(d[addr]);
            }
        }

        Err(BusError::NoDevice)
    }

    fn write(&mut self, addr: Address, data: Data) -> Result<(), BusError> {
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
            Err(BusError::NoDevice)
        } else {
            Ok(())
        }
    }

    fn mount_device(&mut self, device: Box<dyn Device<Address, Data>>) {
        self.devices.push(device);
    }
}

impl<Address, Data> DebugBus<Address, Data>
where
    Address: BitWidth,
    Data: BitWidth,
{
    pub fn new() -> Self {
        Default::default()
        // Self {
        //     address: Arc::new(Mutex::new(Address::from(0))),
        //     data: Arc::new(Mutex::new(Data::from(0))),
        //     devices: Vec::new(),
        // }
    }
}
