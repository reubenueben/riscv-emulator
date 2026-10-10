use crate::dram::*;

pub const DRAM_BASE: u64 = 0x8000_0000;
pub const DRAM_END: u64 = DRAM_SIZE + DRAM_BASE - 1;

pub struct Bus {
    dram: Dram,
}

impl Bus {
    pub fn new(code: Vec<u8>) -> Bus {
        Self {
            dram: Dram::new(code),
        }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, ()> {
        if (DRAM_BASE..=DRAM_END).contains(&addr) {
            self.dram.load(addr, size)
        } else {
            Err(())
        }
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), ()> {
        if (DRAM_BASE..=DRAM_END).contains(&addr) {
            self.dram.store(addr, size, value)
        } else {
            Err(())
        }
    }
}
