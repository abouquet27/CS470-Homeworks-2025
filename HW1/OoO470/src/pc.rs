#[derive(Clone)]
pub struct ProgramCounter {
    pub count: usize,
}

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        return ProgramCounter { count: 0 };
    }
}
