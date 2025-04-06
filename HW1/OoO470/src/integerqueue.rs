use crate::op::OpCode;

const MAX_SIZE: usize = 32;

#[derive(Clone)]
pub struct IssuedInstruction {
    // physical register
    pub destination_register: usize,
    op_a_is_ready: bool,
    op_a_reg_tag: usize,
    pub op_a_value: i64,
    op_b_is_ready: bool,
    op_b_reg_tag: usize,
    pub op_b_value: i64,
    pub opcode: OpCode,
    pub pc: usize,
}

#[derive(Clone)]
pub struct IntegerQueue {
    queue: Vec<IssuedInstruction>,
}

impl IntegerQueue {
    pub fn new() -> IntegerQueue {
        return IntegerQueue { queue: vec![] };
    }
}
