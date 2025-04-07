use std::vec;

use crate::{
    activelist::ActiveList,
    alu::ALU,
    bbt::BusyBitTable,
    dir::DecodedInstructionRegister,
    eflag::ExceptionFlag,
    freelist::FreeList,
    integerqueue::IntegerQueue,
    op::{Instruction, IssuedInstruction, Operand, RenamedInstruction},
    pc::ProgramCounter,
    physregfile::PhysicalRegisterFile,
    rmt::RegisterMapTable,
};

const PIPELINE_SIZE: usize = 4;
#[derive(Clone)]
pub struct CPU {
    // Units
    pc: ProgramCounter,
    register_file: PhysicalRegisterFile,
    dir: DecodedInstructionRegister,
    eflag: ExceptionFlag,
    free_list: FreeList,
    map_table: RegisterMapTable,
    busy_bit_table: BusyBitTable,
    active_list: ActiveList,
    int_queue: IntegerQueue,
    alus: [ALU; PIPELINE_SIZE],

    // Data
    instructions: Vec<Instruction>,
    statelogs: Vec<CPU>,
    cycle_count: usize,
    exception_mode: bool,
}

impl CPU {
    pub fn from_instructions(instructions: Vec<Instruction>) -> CPU {
        return CPU {
            // Units
            pc: ProgramCounter::new(),
            register_file: PhysicalRegisterFile::new(),
            dir: DecodedInstructionRegister::new(),
            eflag: ExceptionFlag::new(),
            free_list: FreeList::new(),
            map_table: RegisterMapTable::new(),
            busy_bit_table: BusyBitTable::new(),
            active_list: ActiveList::new(),
            int_queue: IntegerQueue::new(),
            alus: [ALU::new(), ALU::new(), ALU::new(), ALU::new()],
            // Data
            instructions: instructions,
            statelogs: vec![],
            cycle_count: 0,
            exception_mode: false,
        };
    }

    pub fn dump_state_into_log(&mut self) {
        let cloned_state = self.clone();
        self.statelogs.push(cloned_state);
    }

    pub fn is_finished(&self) -> bool {
        return self.instructions.len() <= self.pc.get_count();
    }

    pub fn is_backpressure(&self) -> bool {
        return self.active_list.is_full() || self.free_list.is_empty() || self.int_queue.is_full();
    }

    // Fetch instructions, update PC and give them to DIR
    // Returns the instruction previously stored by DIR to be processed in R&D (stage 2)
    pub fn fetch_and_decode(&mut self) -> Vec<Instruction> {
        if self.is_backpressure() {
            return vec![];
        };
        let mut fetched_instruction: Vec<Instruction> = vec![];

        for i in 0..PIPELINE_SIZE {
            if !self.is_finished() {
                let new_instruction = self.instructions[self.pc.get_count()];
                self.pc.increment();

                fetched_instruction.push(new_instruction);
            }
        }

        self.dir.add_and_flush_instructions(fetched_instruction)
    }

    pub fn rename_and_dispatch(&mut self, new_instructions: Vec<Instruction>) {
        let mut renamed_instructions: Vec<RenamedInstruction> = vec![];

        for mut instruction in new_instructions {
            // todo add commit register

            // Read operands a and b
            instruction.op_a = self.map_table.get_value(instruction.op_a);

            let op_b = instruction.op_b;
            match op_b {
                Operand::LogicalRegister { id } => {
                    instruction.op_b = Operand::LogicalRegister {
                        id: self.map_table.get_value(id),
                    }
                }
                _ => {}
            };

            // Prepare destination register (mapping)
            let new_physical_reg_dest = self.free_list.pop();

            let old_physical_reg_dest = self
                .map_table
                .get_and_set_mapping(instruction.dest, new_physical_reg_dest);

            instruction.dest = old_physical_reg_dest;

            // Busy bit set
            self.busy_bit_table.set_busy_bit(old_physical_reg_dest);

            let mut issued_instruction = IssuedInstruction::from_instruction(instruction);
        }
    }
}
