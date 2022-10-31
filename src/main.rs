use std::{
    fs::{self, File},
    io::stdin,
    path::PathBuf,
    thread::sleep,
    time::Duration,
};

use byteorder::ReadBytesExt;
use clap::Parser;

use r6502::{
    device::{Device, Output, Ram},
    R6502,
};

#[derive(Parser)]
struct Args {
    // path to program
    bin: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut cpu: R6502 = R6502::new();
    let mut ram: Ram = Ram::with_address_range(0x0000, 0xFFFF);
    let output = Output::with_address(0xFFFB);

    ram.set(0xFFFC, 0x00).unwrap();
    ram.set(0xFFFD, 0x00).unwrap();

    let mut bin = File::open(&args.bin).expect("Failed to open file");

    for addr in 0..fs::metadata(args.bin).unwrap().len() {
        ram.set(addr as u16, bin.read_u8().unwrap())
            .expect("Failed to load program");
    }

    cpu.mount_device(Box::new(ram));
    cpu.mount_device(Box::new(output));

    let mut buf = String::new();

    loop {
        cpu.clock().unwrap();

        stdin().read_line(&mut buf).unwrap();
    }
}
