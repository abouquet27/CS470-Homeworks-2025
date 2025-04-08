const PRF_SIZE: usize = 64;
#[derive(Clone)]
pub struct PhysicalRegisterFile {
    registers: [i64; PRF_SIZE],
}

impl PhysicalRegisterFile {
    pub fn new() -> PhysicalRegisterFile {
        return PhysicalRegisterFile {
            registers: [0; PRF_SIZE],
        };
    }

    pub fn read_register(&self, index: usize) -> i64 {
        self.registers[index]
    }
}
