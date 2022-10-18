pub mod bus;
pub mod core;
pub mod device;
pub mod r6502;
pub use r6502::R6502;

use bus::DebugBus as GenericDebugBus;
pub type DebugBus = GenericDebugBus<u16, u8>;
