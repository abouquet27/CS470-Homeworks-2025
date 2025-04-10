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

    pub fn trigger_exception(&mut self, exception_pc: usize) {
        self.exception = true;
        self.exception_pc = exception_pc;
    }

    pub fn is_exception(&self) -> bool {
        self.exception
    }
}
