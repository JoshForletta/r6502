use crate::{
    device::{Device, Ram},
    r6502::PS,
    R6502,
};

#[derive(Default)]
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

pub fn test_parameterized_cpu_state(tests: &[(&[u8], CpuState, usize)]) {
    for (i, cs, cc) in tests {
        test_cpu_state(i, cs, *cc)
    }
}

pub fn test_cpu_state(instructions: &[u8], cpu_state: &CpuState, clock_cycles: usize) {
    let mut cpu = R6502::new();

    let mut mem = Ram::with_address_range(0x0000, 0xFFFF);

    mem.set(0xFFFC, 0x00).unwrap();
    mem.set(0xFFFD, 0x00).unwrap();

    for (i, instruction) in instructions.iter().enumerate() {
        mem.set(
            TryInto::<u16>::try_into(i).expect("Instructions exceeded address bit width"),
            *instruction,
        )
        .unwrap();
    }

    cpu.mount_device(Box::new(mem));

    for _ in 0..clock_cycles {
        cpu.clock().unwrap();
    }

    if let Some(ps) = cpu_state.ps {
        println!("Prcessor State: ");
        assert_eq!(cpu.ps, ps);
        println!("ok\n");
    }

    if let Some(a) = cpu_state.a {
        println!("Accumulator: ");
        assert_eq!(cpu.a, a);
        println!("ok\n");
    }

    if let Some(x) = cpu_state.x {
        println!("X register: ");
        assert_eq!(cpu.x, x);
        println!("ok\n");
    }

    if let Some(y) = cpu_state.y {
        println!("Y register: ");
        assert_eq!(cpu.y, y);
        println!("ok\n");
    }

    if let Some(pc) = cpu_state.pc {
        println!("Prgram counter: ");
        assert_eq!(cpu.pc, pc);
        println!("ok\n");
    }

    if let Some(sp) = cpu_state.sp {
        println!("Stack pointer: ");
        assert_eq!(cpu.sp, sp);
        println!("ok\n");
    }

    if let Some(extra_cycles) = cpu_state.extra_cycles {
        println!("Extra cycles: ");
        assert_eq!(cpu.extra_cycles, extra_cycles);
        println!("ok\n");
    }

    if let Some(null_pointer) = cpu_state.null_pointer {
        println!("Null pointer: ");
        assert_eq!(cpu.null_pointer, null_pointer);
        println!("ok\n");
    }
}
