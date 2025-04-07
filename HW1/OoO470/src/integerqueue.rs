use crate::op::IssuedInstruction;

const MAX_SIZE: usize = 32;

#[derive(Clone)]
pub struct IntegerQueue {
    queue: Vec<IssuedInstruction>,
    count: usize,
}

impl IntegerQueue {
    pub fn new() -> IntegerQueue {
        return IntegerQueue {
            queue: vec![],
            count: 0,
        };
    }

    pub fn is_full(&self) -> bool {
        return self.count >= MAX_SIZE;
    }
}
