const RMT_SIZE: usize = 32;
#[derive(Clone)]
pub struct RegisterMapTable {
    mapping: [usize; RMT_SIZE],
}

impl RegisterMapTable {
    pub fn new() -> RegisterMapTable {
        let mut mapping = [0; RMT_SIZE];

        for i in 0..RMT_SIZE {
            mapping[i] = i;
        }

        return RegisterMapTable { mapping };
    }
}
