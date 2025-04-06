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
}
