use std::fmt::UpperHex;

use crate::{
    device::{Device, Ram},
    r6502::PS,
    R6502,
};

#[derive(Debug, Default)]
pub struct CpuState {
    pub ps: Option<PS>,
    pub a: Option<u8>,
    pub x: Option<u8>,
    pub y: Option<u8>,
    pub pc: Option<u16>,
    pub sp: Option<u8>,
    pub extra_cycles: Option<u8>,
    pub null_pointer: Option<u8>,
}

#[derive(Debug)]
pub struct EmulationStateTest<'a> {
    pub instructions: &'a [u8],      // instructions
    pub initial_cpu_state: CpuState, // initial cpu state
    pub test_cpu_state: CpuState,    // test cpu state
    pub mem_tests: &'a [(u16, u8)],
    pub clock_cycles: usize,
}

impl<'a> Default for EmulationStateTest<'a> {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            initial_cpu_state: Default::default(),
            test_cpu_state: Default::default(),
            mem_tests: Default::default(),
            clock_cycles: 2,
        }
    }
}

fn compare<T: PartialEq + UpperHex>(name: &str, actual: T, expected: T) -> bool {
    println!("{}:", name);
    println!("  expected: ${:X}", expected);
    println!("    actual: ${:X}\n", actual);

    return expected == actual;
}

pub fn test_emulation_state(est: &EmulationStateTest) {
    let mut cpu = R6502 {
        ps: est.initial_cpu_state.ps.unwrap_or(Default::default()),
        a: est.initial_cpu_state.a.unwrap_or(Default::default()),
        x: est.initial_cpu_state.x.unwrap_or(Default::default()),
        y: est.initial_cpu_state.y.unwrap_or(Default::default()),
        pc: est.initial_cpu_state.pc.unwrap_or(Default::default()),
        sp: est.initial_cpu_state.sp.unwrap_or(Default::default()),
        bus: Default::default(),
        extra_cycles: est
            .initial_cpu_state
            .extra_cycles
            .unwrap_or(Default::default()),
        null_pointer: est
            .initial_cpu_state
            .null_pointer
            .unwrap_or(Default::default()),
    };

    let mut mem = Ram::with_address_range(0x0000, 0xFFFF);

    mem.set(0xFFFC, 0x00).unwrap();
    mem.set(0xFFFD, 0x00).unwrap();

    for (i, instruction) in est.instructions.iter().enumerate() {
        mem.set(
            TryInto::<u16>::try_into(i).expect("Instructions exceeded address bit width"),
            *instruction,
        )
        .unwrap();
    }

    cpu.mount_device(Box::new(mem));

    for _ in 0..est.clock_cycles {
        cpu.clock().unwrap();
    }

    let mut passed = true;

    println!("{:#?}\n", est);
    println!("{:#?}\n", cpu);

    if let Some(ps) = est.test_cpu_state.ps {
        passed = compare("Processor status", cpu.ps, ps)
    }

    if let Some(a) = est.test_cpu_state.a {
        passed = compare("Accumulator", cpu.a, a);
    }

    if let Some(x) = est.test_cpu_state.x {
        passed = compare("X register", cpu.x, x);
    }

    if let Some(y) = est.test_cpu_state.y {
        passed = compare("Y register", cpu.y, y);
    }

    if let Some(pc) = est.test_cpu_state.pc {
        passed = compare("Program counter", cpu.pc, pc);
    }

    if let Some(sp) = est.test_cpu_state.sp {
        passed = compare("Stack pointer", cpu.sp, sp);
    }

    if let Some(extra_cycles) = est.test_cpu_state.extra_cycles {
        passed = compare("Extra cycles", cpu.extra_cycles, extra_cycles);
    }

    if let Some(null_pointer) = est.test_cpu_state.null_pointer {
        passed = compare("null_pointer", cpu.null_pointer, null_pointer);
    }

    for (addr, data) in est.mem_tests {
        passed = compare(
            format_args!("Address: ${:X}, Data: ${:X}", addr, data)
                .as_str()
                .unwrap(),
            cpu.bus.read(*addr).unwrap(),
            *data,
        );
    }

    if !passed {
        panic!();
    }
}
