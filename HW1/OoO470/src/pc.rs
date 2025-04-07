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
}
