use crate::op::RenamedInstruction;

const MAX_SIZE: usize = 32;

#[derive(Clone)]
pub struct ActiveList {
    renamed_instructions: Vec<RenamedInstruction>,
    count: usize,
}

impl ActiveList {
    pub fn new() -> ActiveList {
        let renamed_instructions = vec![];
        return ActiveList {
            renamed_instructions,
            count: 0,
        };
    }

    pub fn is_full(&self) -> bool {
        self.count >= 32
    }

    pub fn push_instruction(&mut self, instruction: RenamedInstruction) -> Result<(), &str> {
        if !self.is_full() {
            return Err("Trying to add an instruction but the list is full");
        }

        self.renamed_instructions.push(instruction);
        self.count += 1;
        Ok(())
    }
}
