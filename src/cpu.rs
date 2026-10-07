pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,
    pub code: Vec<u8>,
}

impl Cpu {
    pub fn dump_registers(&self) {
        for (i, r) in self.regs.iter().enumerate() {
            println!("x{:<2} = {:#018x}", i, r);
        }
    }

    pub fn new(code: Vec<u8>) -> Self {
        let mut regs = [0; 32];
        regs[2] = code.len() as u64;
        Self { regs, pc: 0, code }
    }

    pub fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        return (self.code[index] as u32)
            | ((self.code[index + 1] as u32) << 8)
            | ((self.code[index + 2] as u32) << 16)
            | ((self.code[index + 3] as u32) << 24);
    }

    pub fn execute(&mut self, inst: u32) {
        let opcode = inst & 0x7f; // check first 7 bits for operation code
        let rd = ((inst >> 7) & 0x1f) as usize; // shifts bits to the right by 7 to discard
        // opcode, then gets destination register
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;

        self.regs[0] = 0;

        match opcode {
            0x13 => {
                // addi

                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64; // converts 12 bit signed value
                // to 64 bit
                self.regs[rd] = self.regs[rs1].wrapping_add(imm);
            }
            0x33 => {
                // add

                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
            }
            _ => {
                dbg!(format!("not implemented {:#} yet", opcode));
            }
        }
    }
}
