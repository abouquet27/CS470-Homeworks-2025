#[derive(Clone)]
pub struct DecodedInstructionRegister {
    decoded_pcs: Vec<i64>,
}

impl DecodedInstructionRegister {
    pub fn new() -> DecodedInstructionRegister {
        return DecodedInstructionRegister {
            decoded_pcs: vec![],
        };
    }
}
