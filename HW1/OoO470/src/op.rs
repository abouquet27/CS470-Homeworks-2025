#[derive(Debug, Clone)]
pub enum OpCode {
    ADD,
    ADDI,
    SUB,
    MULU,
    DIVU,
    REMU,
}

#[derive(Debug, Clone)]
enum Operand {
    LogicalRegister { name: String, id: usize },
    Imm { value: i64 },
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: OpCode,
    pub dest: Operand,
    pub op_a: Operand,
    pub op_b: Operand,
    pub result: Option<i64>,
}

impl Instruction {
    pub fn from_str(s: String) -> Instruction {
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

        let dest: Operand = Operand::LogicalRegister {
            name: parsed_str[1].to_string(),
            id: parsed_str[1][1..].parse::<usize>().unwrap(),
        };

        let op_a: Operand = Operand::LogicalRegister {
            name: parsed_str[2].to_string(),
            id: parsed_str[1][1..].parse::<usize>().unwrap(),
        };

        let op_b: Operand = match opcode {
            OpCode::ADDI => {
                let imm = parsed_str[3].parse::<i64>().unwrap();
                Operand::Imm { value: imm }
            }
            _ => Operand::LogicalRegister {
                name: parsed_str[3].to_string(),
                id: parsed_str[1][1..].parse::<usize>().unwrap(),
            },
        };

        let instruction = Instruction {
            opcode,
            dest,
            op_a,
            op_b,
            result: None,
        };

        println!("{:#?}", instruction);

        return instruction;
    }
}
