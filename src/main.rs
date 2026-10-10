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

    loop {
        let inst = match cpu.fetch() {
            Ok(inst) => inst,
            Err(_) => break,
        };

        cpu.pc += 4;

        match cpu.execute(inst) {
            Ok(_) => {}
            Err(_) => break,
        };

        if cpu.pc == 0 {
            break;
        }
    }
    cpu.dump_registers();
    Ok(())
}
