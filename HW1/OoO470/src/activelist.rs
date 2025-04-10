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

    pub fn length(&self) -> usize {
        self.count
    }

    pub fn get_entries(&self) -> Vec<ActiveListEntry> {
        self.entries.clone()
    }

    pub fn set_entries(&mut self, new_entries: Vec<ActiveListEntry>) {
        self.entries = new_entries;
    }

    fn push_entry(&mut self, entry: ActiveListEntry) {
        self.entries.push(entry);
        self.count += 1;
    }

    fn find_instruction_index(&mut self, pc: usize) -> Option<usize> {
        self.entries.iter().position(|&e| e.pc == pc)
    }

    pub fn append_entries(&mut self, entries: Vec<ActiveListEntry>) {
        for entry in entries {
            self.push_entry(entry);
        }
    }

    pub fn update_instruction(&mut self, forwarded_result: Result<Option<(usize, i64)>, usize>) {
        match forwarded_result {
            Ok(opt) => {
                // No exception

                if let Some((pc, _)) = opt {
                    // The instruction was valid and not a nop
                    if let Some(index) = self.find_instruction_index(pc) {
                        // the instruction is in the active entry
                        self.entries[index].done = true // set up done.
                    }
                }
            }
            Err(pc) => {
                // There is an exception
                if let Some(index) = self.find_instruction_index(pc) {
                    // the instruction is in the active entry
                    self.entries[index].done = true; // set up done.
                    self.entries[index].exception = true // set up done.
                }
            }
        }
    }
}
