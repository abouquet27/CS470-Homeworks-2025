#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    ADD,
    ADDI,
    SUB,
    MULU,
    DIVU,
    REMU,
}

#[derive(Debug, Copy, Clone)]
pub enum Operand {
    LogicalRegister { id: usize },
    Imm { value: i64 },
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub opcode: OpCode,
    pub dest: usize,
    pub op_a: usize,
    pub op_b: Operand,
    pub pc: usize,
}

impl Instruction {
    pub fn from_str(s: &str, pc: usize) -> Instruction {
        let binding = s.replace(",", "");
        let parsed_str: Vec<&str> = binding.split(' ').collect();

        let opcode: OpCode = match parsed_str[0] {
            "add" => OpCode::ADD,
            "addi" => OpCode::ADDI,
            "sub" => OpCode::SUB,
            "mulu" => OpCode::MULU,
            "divu" => OpCode::DIVU,
            "remu" => OpCode::REMU,
            _ => panic!("Unknown instruction"),
        };

        let dest = parsed_str[1][1..].parse::<usize>().unwrap();

        let op_a = parsed_str[2][1..].parse::<usize>().unwrap();

        let op_b: Operand = match opcode {
            OpCode::ADDI => {
                let imm = parsed_str[3].parse::<i64>().unwrap();
                Operand::Imm { value: imm }
            }
            _ => Operand::LogicalRegister {
                id: parsed_str[3][1..].parse::<usize>().unwrap(),
            },
        };

        let instruction = Instruction {
            opcode,
            dest,
            op_a,
            op_b,
            pc,
        };

        println!("{:#?}", instruction);

        return instruction;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ActiveListEntry {
    done: bool,
    exception: bool,
    logical_destination: usize,
    old_destination: usize,
    pc: usize,
}

impl ActiveListEntry {
    pub fn from_instruction(instruction: Instruction, old_destination: usize) -> ActiveListEntry {
        ActiveListEntry {
            done: false,
            exception: false,
            logical_destination: instruction.dest,
            old_destination: old_destination,
            pc: instruction.pc,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct IssuedInstruction {
    // physical register
    pub destination_register: usize,
    pub op_a_is_ready: bool,
    pub op_a_reg_tag: usize,
    pub op_a_value: i64,
    pub op_b_is_ready: bool,
    pub op_b_reg_tag: usize,
    pub op_b_value: i64,
    pub opcode: OpCode,
    pub pc: usize,
}

impl IssuedInstruction {
    pub fn from_instruction(
        instruction: Instruction,
        physical_destination: usize,
        physical_op_a: usize,
        physical_op_b: usize,
        op_a_is_ready: bool,
        op_b_is_ready: bool,
        op_a_value: i64,
        op_b_value: i64,
    ) -> IssuedInstruction {
        return IssuedInstruction {
            destination_register: physical_destination,
            op_a_is_ready: op_a_is_ready,
            op_a_reg_tag: physical_op_a,
            op_a_value: op_a_value,
            op_b_is_ready: op_b_is_ready,
            op_b_reg_tag: physical_op_b,
            op_b_value: op_b_value,
            opcode: instruction.opcode,
            pc: instruction.pc,
        };
    }

    pub fn is_ready(&self) -> bool {
        self.op_a_is_ready && self.op_b_is_ready
    }

    pub fn set_op_a(&mut self, new_op_a_value: i64) {
        self.op_a_value = new_op_a_value;
        self.op_a_is_ready = true
    }

    pub fn set_op_b(&mut self, new_op_b_value: i64) {
        self.op_b_value = new_op_b_value;
        self.op_b_is_ready = true
    }
}
