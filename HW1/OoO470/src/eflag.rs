use crate::pc::ProgramCounter;

#[derive(Clone)]
pub struct ExceptionFlag {
    exception_pc: usize,
    exception: bool,
}

impl ExceptionFlag {
    pub fn new() -> ExceptionFlag {
        return ExceptionFlag {
            exception_pc: 0,
            exception: false,
        };
    }

    pub fn trigger_exception(&mut self, pc: &mut ProgramCounter) {
        self.exception_pc = pc.count;
        self.exception = true;
        pc.count = 0x10000;
    }
}
