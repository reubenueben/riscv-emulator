mod bus;
mod cpu;
mod dram;
use crate::cpu::*;

use std::{
    env,
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: riscv-emulator <filename>");
    }
    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    let mut cpu = Cpu::new(code);

    while cpu.pc < cpu.code.len() as u64 {
        let inst = cpu.fetch();
        cpu.pc += 4;
        cpu.execute(inst);
    }
    cpu.dump_registers();
    Ok(())
}
