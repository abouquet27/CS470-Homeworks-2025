use std::sync::BarrierWaitResult;

use serde_json::de;

const RMT_SIZE: usize = 32;
#[derive(Clone)]
pub struct RegisterMapTable {
    // entry id > physical register
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

    pub fn get_value(&self, logical_dest: usize) -> usize {
        self.mapping[logical_dest]
    }

    pub fn get_and_set_mapping(&mut self, logical_dest: usize, new_mapping: usize) -> usize {
        let old_mapping = self.mapping[logical_dest];
        self.mapping[logical_dest] = new_mapping;
        old_mapping
    }
}
