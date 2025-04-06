const FIRST_FREE_REG: usize = 32;

#[derive(Clone)]
pub struct FreeList {
    queue: Vec<usize>,
}

impl FreeList {
    pub fn new() -> FreeList {
        let mut queue = vec![];

        for i in FIRST_FREE_REG..2 * FIRST_FREE_REG {
            queue.push(i);
        }

        return FreeList { queue };
    }
}
