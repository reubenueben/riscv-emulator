pub const DRAM_SIZE: u64 = 1024 * 1024 * 1;

pub struct Dram {
    pub dram: Vec<u8>,
}

impl Dram {
    pub fn new(code: Vec<u8>) -> Self {
        let mut dram = vec![0; DRAM_SIZE as usize];
        dram.splice(..code.len(), code.iter().cloned());

        Self { dram }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, ()> {
        match size {
            8 => Ok(self.load8(addr)),
            16 => Ok(self.load16(addr)),
            32 => Ok(self.load32(addr)),
            64 => Ok(self.load64(addr)),
            _ => Err(()),
        }
    }

    pub fn store(&self, addr: u64, size: u64, value: u64) -> Result<(), ()> {
        match size {
            8 => Ok(self.store8(addr, value)),
            16 => Ok(self.store16(addr, value)),
            32 => Ok(self.store32(addr, value)),
            64 => Ok(self.store64(addr, value)),
            _ => Err(()),
        }
    }

    fn load8(&self, addr: u64) -> u64 {
        3
    }

    fn load16(&self, addr: u64) -> u64 {
        3
    }

    fn load32(&self, addr: u64) -> u64 {
        3
    }

    fn load64(&self, addr: u64) -> u64 {
        3
    }

    fn store8(&self, addr: u64, value: u64) {
        ()
    }

    fn store16(&self, addr: u64, value: u64) {
        ()
    }

    fn store32(&self, addr: u64, value: u64) {
        let index = ();
        ()
    }

    fn store64(&self, addr: u64, value: u64) {
        ()
    }
}
