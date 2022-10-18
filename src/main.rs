use r6502::{
    device::{Device, Ram},
    R6502,
};

fn main() {
    let mut cpu: R6502 = R6502::new();
    let mut ram: Ram = Ram::with_address_range(0xFF00, 0xFFFF);

    ram.set(0xfffc, 0x00).unwrap();
    ram.set(0xfffd, 0xff).unwrap();
    ram.set(0xff00, 0x01).unwrap(); // temp LDA IMM ins
    ram.set(0xff01, 0x00).unwrap();
    ram.set(0xff02, 0xff).unwrap();

    cpu.mount_device(Box::new(ram));

    cpu.reset();

    // dbg!(&cpu);

    cpu.clock().unwrap();

    dbg!(cpu);
}
