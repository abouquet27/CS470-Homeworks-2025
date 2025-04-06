const MAX_SIZE: usize = 32;

#[derive(Clone)]
pub struct RenamedInstruction {
    done: bool,
    exception: bool,
    logical_destination: usize,
    old_destination: usize,
    pc: usize,
}

#[derive(Clone)]
pub struct ActiveList {
    renamed_instructions: Vec<RenamedInstruction>,
}

impl ActiveList {
    pub fn new() -> ActiveList {
        let renamed_instructions = vec![];
        return ActiveList {
            renamed_instructions,
        };
    }
}
