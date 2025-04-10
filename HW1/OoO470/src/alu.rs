use crate::op::{IssuedInstruction, OpCode};

#[derive(Clone)]
pub struct ALU {
    new_instruction: Option<IssuedInstruction>,
    current_instruction: Option<IssuedInstruction>,
    result_previous_instruction: Result<Option<(usize, i64)>, usize>,
}

impl ALU {
    pub fn new() -> ALU {
        return ALU {
            new_instruction: None,
            current_instruction: None,
            result_previous_instruction: Ok(None),
        };
    }

    pub fn add_instruction(&mut self, new_instr: Option<IssuedInstruction>) {
        self.current_instruction = self.new_instruction; // erase the instruction just computed
        self.new_instruction = new_instr;
    }

    pub fn process_current_instruction(&mut self) -> Result<Option<(usize, i64)>, usize> {
        let processed_instruction = self.current_instruction.clone();

        let result: Result<Option<(usize, i64)>, usize> =
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
                            Err(instruction.pc)
                        } else {
                            Ok(Some((
                                instruction.destination_register,
                                instruction.op_a_value / instruction.op_b_value,
                            )))
                        }
                    }
                    OpCode::REMU => {
                        if instruction.op_b_value == 0 {
                            Err(instruction.pc)
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

        self.result_previous_instruction = result.clone();
        result
    }

    pub fn clear(&mut self) {
        self.current_instruction = None;
        self.new_instruction = None;
        self.result_previous_instruction = Ok(None)
    }
}
