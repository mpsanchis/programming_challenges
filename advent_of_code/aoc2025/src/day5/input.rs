use std::fs;
use super::types::{ID, Range};


// Note: optimise by reading only part of the file containing ranges, returning pointer, and then reading only the second part of the file
pub fn parse_input() -> (Vec<Range>, Vec<ID>) {
    let input = fs::read_to_string(file!().replace(".rs", ".txt")).expect("could not read input");

    let mut ranges: Vec<Range> = Vec::new();
    let mut ids: Vec<ID> = Vec::new();

    for line in input.lines() {
        if line.contains('-') {
            let range: Vec<ID> = line.split('-').map(|id| id.parse().unwrap()).collect();
            ranges.push(Range { start: range[0], end: range[1]} );
        } else if line != "" {
            ids.push(line.parse().unwrap());
        }
    }
    (ranges, ids)
}