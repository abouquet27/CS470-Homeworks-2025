use crate::{
    activelist::{self, ActiveList},
    alu::ALU,
    bbt::{self, BusyBitTable},
    dir::DecodedInstructionRegister,
    eflag::ExceptionFlag,
    freelist::FreeList,
    integerqueue::IntegerQueue,
    op::{ActiveListEntry, Instruction, IssuedInstruction, Operand},
    pc::ProgramCounter,
    physregfile::PhysicalRegisterFile,
    rmt::RegisterMapTable,
};

const PIPELINE_SIZE: usize = 4;
const DEBUG: bool = true;
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
    integer_queue: IntegerQueue,
    alus: [ALU; PIPELINE_SIZE],

    // Data
    instructions: Vec<Instruction>,
    statelogs: Vec<CPU>,
    cycle_count: usize,
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
            integer_queue: IntegerQueue::new(),
            alus: [ALU::new(), ALU::new(), ALU::new(), ALU::new()],
            // Data
            instructions: instructions,
            statelogs: vec![],
            cycle_count: 0,
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
        return self.active_list.is_full()
            || self.free_list.is_empty()
            || self.integer_queue.is_full();
    }

    // Fetch instructions, update PC and give them to DIR
    // Returns the instruction previously stored by DIR to be processed in R&D (stage 2)
    pub fn fetch_and_decode(&mut self) -> Vec<Instruction> {
        if self.is_backpressure() || self.eflag.is_exception() {
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
        let mut new_issued_instructions: Vec<IssuedInstruction> = vec![];
        let mut new_active_list_entries: Vec<ActiveListEntry> = vec![];

        for instruction in new_instructions {
            // Checks operand a and determines if it's ready to be read.
            let physical_op_a = self.map_table.get_value(instruction.op_a);
            let op_a_is_ready = self.busy_bit_table.get_busy_bit(physical_op_a);
            let op_a_value = if op_a_is_ready {
                self.register_file.read_register(physical_op_a)
            } else {
                0
            };

            // Checks operand b and determines if it's ready to be read.
            let (physical_op_b, op_b_is_ready, op_b_value) = match instruction.op_b {
                Operand::LogicalRegister { id } => {
                    let phys_reg = self.map_table.get_value(id);
                    let is_ready = self.busy_bit_table.get_busy_bit(phys_reg);
                    let value = if is_ready {
                        self.register_file.read_register(phys_reg)
                    } else {
                        0
                    };
                    (phys_reg, is_ready, value)
                }
                Operand::Imm { value } => (0, true, value),
            };

            // Prepare destination register (mapping)
            let physical_destination = self.free_list.pop();

            let old_physical_reg_dest = self
                .map_table
                .get_and_set_mapping(instruction.dest, physical_destination);

            // Busy bit set
            self.busy_bit_table.set_busy_bit(physical_destination);

            // Issued instruction created
            let issued_instruction = IssuedInstruction::from_instruction(
                instruction,
                physical_destination,
                physical_op_a,
                physical_op_b,
                op_a_is_ready,
                op_b_is_ready,
                op_a_value,
                op_b_value,
            );

            new_issued_instructions.push(issued_instruction);

            let active_list_entry =
                ActiveListEntry::from_instruction(instruction, old_physical_reg_dest);

            new_active_list_entries.push(active_list_entry);
        }

        self.active_list.append_entries(new_active_list_entries);
        self.integer_queue
            .append_instructions(new_issued_instructions);
    }

    pub fn issuance_stage(&mut self) {
        let instructions = self.integer_queue.fetch_ready_instruction();

        // set up the ALUS

        for i in 0..PIPELINE_SIZE {
            let opt_instr = if i < instructions.len() {
                Some(instructions[i])
            } else {
                None
            };

            self.alus[i].add_instruction(opt_instr);
        }
    }

    pub fn commit_stage(&mut self, forwarded_results: Vec<Result<Option<(usize, i64)>, usize>>) {
        // First part:
        // Going through ActiveList's entries and checking them.

        let mut count = 0;
        let mut no_more_instruction = false;
        let mut removed_entries: Vec<ActiveListEntry> = vec![];
        let mut kept_entries: Vec<ActiveListEntry> = self.active_list.get_entries();

        while let Some(e) = kept_entries.first() {
            if e.done || e.exception || PIPELINE_SIZE <= count {
                // the older instruction is not taken because:
                // 1. it's not done (the committed instruction must preserver the program order)
                // 2. it triggers an exception
                // 3. 4 instructions have already been removed

                if e.exception {
                    self.eflag.trigger_exception(e.pc);
                    self.pc.error_pc();
                }

                break;
            }

            let removed_entry = kept_entries.remove(0);
            removed_entries.push(removed_entry);
        }

        self.active_list.set_entries(kept_entries);

        // Second part:
        // Updating ActiveList's entries from the result

        for result in forwarded_results {
            self.active_list.update_instruction(result);
        }

        if DEBUG {
            println!("Removed entries: {:#?}", removed_entries);
        }
    }

    pub fn exception_recovery_mode(&mut self) {
        self.dir.clear();
        self.integer_queue.clear();
        for i in 0..PIPELINE_SIZE {
            self.alus[i].clear();
        }
        
    }
}
