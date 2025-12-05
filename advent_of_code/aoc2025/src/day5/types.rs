use std::cmp::{max, min};
use std::fmt::Debug;

pub type ID = u64;

pub struct Range {
    pub start: ID,
    pub end: ID,
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}

impl Copy for Range {}

impl Clone for Range {
    fn clone(&self) -> Self {
        *self
    }
}

impl Range {
    pub fn size(&self) -> u64 {
        self.end - self.start + 1
    }

    pub fn contains_id(&self, id: ID) -> bool {
        id >= self.start && id <= self.end
    }

    pub fn merge_ranges(r1: &Range, r2: &Range) -> Option<Range> {
        if r2.start > r1.end || r2.end < r1.start {
            return None;
        }
        Some(Range {
            start: min(r1.start, r2.start),
            end: max(r1.end, r2.end),
        })
    }
}