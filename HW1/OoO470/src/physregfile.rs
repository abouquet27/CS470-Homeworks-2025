const PRF_SIZE: usize = 64;
#[derive(Clone)]
pub struct PhysicalRegisterFile {
    values: [i64; PRF_SIZE],
}

impl PhysicalRegisterFile {
    pub fn new() -> PhysicalRegisterFile {
        return PhysicalRegisterFile {
            values: [0; PRF_SIZE],
        };
    }
}
