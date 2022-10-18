use r6502::{device::Ram, DebugBus, R6502};

fn main() {
    let mut cpu: R6502<DebugBus> = R6502::new();
    let ram: Ram<u16, u8> = Ram::with_address_range(0xFF00, 0xFFFF);

    cpu.mount_device(Box::new(ram));

    cpu.reset();

    dbg!(cpu);
}
