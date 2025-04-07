use crate::op::Instruction;

#[derive(Clone, Debug)]
pub struct DecodedInstructionRegister {
    decoded_pcs: Vec<Instruction>,
}

impl DecodedInstructionRegister {
    pub fn new() -> DecodedInstructionRegister {
        return DecodedInstructionRegister {
            decoded_pcs: vec![],
        };
    }

    pub fn add_and_flush_instructions(
        &mut self,
        new_instructions: Vec<Instruction>,
    ) -> Vec<Instruction> {
        
        let decoded_pcs = self.decoded_pcs.clone();
        self.decoded_pcs = new_instructions;

        return decoded_pcs;
    }
}
