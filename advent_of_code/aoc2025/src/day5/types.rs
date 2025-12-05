pub type ID = u64;

pub struct Range {
    pub start: ID,
    pub end: ID,
}

impl Range {
    pub fn contains_id(&self, id: ID) -> bool {
        id >= self.start && id <= self.end
    }
}