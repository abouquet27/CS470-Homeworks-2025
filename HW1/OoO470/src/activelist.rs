use crate::op::ActiveListEntry;

const MAX_SIZE: usize = 32;

#[derive(Clone)]
pub struct ActiveList {
    entries: Vec<ActiveListEntry>,
    count: usize,
}

impl ActiveList {
    pub fn new() -> ActiveList {
        let entries = vec![];
        return ActiveList { entries, count: 0 };
    }

    pub fn is_full(&self) -> bool {
        self.count >= 32
    }

    fn push_entry(&mut self, entry: ActiveListEntry) {
        self.entries.push(entry);
        self.count += 1;
    }

    pub fn append_entries(&mut self, entries: Vec<ActiveListEntry>) {
        for entry in entries {
            self.push_entry(entry);
        }
    }
}
