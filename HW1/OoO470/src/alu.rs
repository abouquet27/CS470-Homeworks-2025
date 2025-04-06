use std::error::Error;

use crate::{
    integerqueue::IssuedInstruction,
    op::{Instruction, OpCode},
};

#[derive(Clone)]
pub struct ALU {
    current_instruction: Option<IssuedInstruction>,
}

impl ALU {
    pub fn new() -> ALU {
        return ALU {
            current_instruction: None,
        };
    }

    pub fn add_and_process_instruction(
        &mut self,
        new_instr: Option<IssuedInstruction>,
    ) -> Result<Option<i64>, &str> {
        let processed_instruction = self.current_instruction.clone();

        let result = if let Some(instruction) = processed_instruction {
            match instruction.opcode {
                OpCode::ADD => Some(instruction.op_a_value + instruction.op_b_value),
                OpCode::ADDI => Some(instruction.op_a_value + instruction.op_b_value),
                OpCode::SUB => Some(instruction.op_a_value - instruction.op_b_value),
                OpCode::MULU => Some(instruction.op_a_value * instruction.op_b_value),
                OpCode::DIVU => {
                    if instruction.op_b_value == 0 {
                        return Err("Divisor operand is null");
                    }
                    Some(instruction.op_a_value / instruction.op_b_value)
                }
                OpCode::REMU => {
                    if instruction.op_b_value == 0 {
                        return Err("Divisor operand is null");
                    }
                    Some(instruction.op_a_value % instruction.op_b_value)
                }
            }
        } else {
            None
        };

        self.current_instruction = new_instr;
        return Ok(result);
    }
}
