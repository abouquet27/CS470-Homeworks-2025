const TABLE_SIZE: usize = 64;

#[derive(Clone)]
pub struct BusyBitTable {
    table: [bool; TABLE_SIZE],
}

impl BusyBitTable {
    pub fn new() -> BusyBitTable {
        return BusyBitTable {
            table: [false; TABLE_SIZE],
        };
    }

    pub fn is_full(&self) -> bool {
        self.table.iter().all(|entry| *entry)
    }

    pub fn set_busy_bit(&mut self, index: usize) {
        self.table[index] = true;
    }
}
