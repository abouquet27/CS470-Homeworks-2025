use std::fs::read;

use crate::{op::IssuedInstruction, physregfile::PhysicalRegisterFile};

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

    fn push_instruction(&mut self, instruction: IssuedInstruction) {
        self.queue.push(instruction);
        self.count += 1;
    }

    pub fn append_instructions(&mut self, instructions: Vec<IssuedInstruction>) {
        for instruction in instructions {
            self.push_instruction(instruction);
        }
    }

    // Update instruction using the results computed by the ALUS
    // on the forwarding path
    pub fn update_instructions(&mut self, forwarded_results: Vec<(usize, i64)>) {
        for i in self.queue.iter_mut() {
            if !i.op_a_is_ready {
                if let Some(r) = forwarded_results
                    .iter()
                    .find(|&&(dest, _)| dest == i.op_a_reg_tag)
                {
                    i.op_a_value = r.1;
                }
            }

            if !i.op_b_is_ready {
                if let Some(r) = forwarded_results
                    .iter()
                    .find(|&&(dest, _)| dest == i.op_b_reg_tag)
                {
                    i.op_b_value = r.1;
                }
            }
        }
    }

    pub fn fetch_ready_instruction(&mut self) -> Vec<IssuedInstruction> {
        let mut ready_instructions: Vec<IssuedInstruction> = vec![];
        let mut iter = self.queue.iter();
        let Some(&rd_instr) = iter.find(|&&i| i.is_ready()) else {
            println!("No instruction is ready");
            return ready_instructions;
        };

        let mut count = 1;
        ready_instructions.push(rd_instr);
        let mut is_valid = true;

        while is_valid && count < 4 {
            if let Some(&rd_i) = iter.next() {
                ready_instructions.push(rd_i);
                count += 1;
            } else {
                is_valid = false;
            };
        }

        self.queue
            .retain(|&i| ready_instructions.iter().all(|&r| r.pc != i.pc));

        ready_instructions
    }

    pub fn clear(&mut self) {
        self.queue = vec![];
        self.count = 0;
    }
}
