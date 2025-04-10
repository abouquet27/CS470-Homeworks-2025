const EXCEPTION_PC_VALUE: usize = 0x10000;

#[derive(Clone)]
pub struct ProgramCounter {
    count: usize,
}

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        return ProgramCounter { count: 0 };
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn error_pc(&mut self) {
        self.count = EXCEPTION_PC_VALUE;
    }
}
