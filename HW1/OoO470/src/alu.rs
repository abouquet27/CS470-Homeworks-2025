use crate::op::{IssuedInstruction, OpCode};

#[derive(Clone)]
pub struct ALU {
    current_instruction: Option<IssuedInstruction>,
    result_previous_instruction: Result<Option<(usize, i64)>, String>,
}

impl ALU {
    pub fn new() -> ALU {
        return ALU {
            current_instruction: None,
            result_previous_instruction: Ok(None),
        };
    }

    pub fn add_and_process_instruction(
        &mut self,
        new_instr: Option<IssuedInstruction>,
    ) -> Result<Option<(usize, i64)>, String> {
        let processed_instruction = self.current_instruction.clone();

        let result: Result<Option<(usize, i64)>, String> =
            if let Some(instruction) = processed_instruction {
                match instruction.opcode {
                    OpCode::ADD => Ok(Some((
                        instruction.destination_register,
                        instruction.op_a_value + instruction.op_b_value,
                    ))),
                    OpCode::ADDI => Ok(Some((
                        instruction.destination_register,
                        instruction.op_a_value + instruction.op_b_value,
                    ))),
                    OpCode::SUB => Ok(Some((
                        instruction.destination_register,
                        instruction.op_a_value - instruction.op_b_value,
                    ))),
                    OpCode::MULU => Ok(Some((
                        instruction.destination_register,
                        instruction.op_a_value * instruction.op_b_value,
                    ))),
                    OpCode::DIVU => {
                        if instruction.op_b_value == 0 {
                            Err("Divisor operand is null".to_string())
                        } else {
                            Ok(Some((
                                instruction.destination_register,
                                instruction.op_a_value / instruction.op_b_value,
                            )))
                        }
                    }
                    OpCode::REMU => {
                        if instruction.op_b_value == 0 {
                            Err("Divisor operand is null".to_string())
                        } else {
                            Ok(Some((
                                instruction.destination_register,
                                instruction.op_a_value % instruction.op_b_value,
                            )))
                        }
                    }
                }
            } else {
                Ok(None)
            };

        self.current_instruction = new_instr;
        self.result_previous_instruction = result.clone();
        result
    }
}
