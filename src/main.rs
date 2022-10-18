use r6502::{bus::DebugBus, device::Ram, R6502};

fn main() {
    let mut cpu: R6502<DebugBus> = R6502::new();
    let ram: Ram = Ram::with_address_range(0xFF00, 0xFFFF);

    cpu.mount_device(Box::new(ram));

    cpu.reset();

    dbg!(cpu);
}
