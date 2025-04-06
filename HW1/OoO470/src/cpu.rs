use std::vec;

use crate::{
    activelist::ActiveList, alu::ALU, bbt::BusyBitTable, dir::DecodedInstructionRegister,
    eflag::ExceptionFlag, freelist::FreeList, integerqueue::IntegerQueue, op::Instruction,
    pc::ProgramCounter, physregfile::PhysicalRegisterFile, rmt::RegisterMapTable,
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
        };
    }

    pub fn dump_state_into_log(&mut self) {
        let cloned_state = self.clone();
        self.statelogs.push(cloned_state);
    }

    pub fn is_finished(&self) -> bool {
        return self.instructions.len() <= self.pc.count;
    }
}
