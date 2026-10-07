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
}
