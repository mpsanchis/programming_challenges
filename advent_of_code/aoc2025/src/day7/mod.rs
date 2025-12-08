use std::collections::HashSet;

use crate::util::{Part, logger};

mod input;
mod types;

pub fn main(part: &Part) {
    let splitters_per_line = input::parse_input();
    let mut beam_positions: HashSet<usize> = HashSet::new();
    beam_positions.insert(splitters_per_line[0][0]);
    
    let mut number_of_splits = 0;
    for splitters in &splitters_per_line[1..] {
        logger().logn(&format!("Splitters: {:?}", splitters));
        for splitter in splitters {
            if beam_positions.contains(splitter) {
                number_of_splits += 1;
                beam_positions.remove(splitter);
                beam_positions.insert(splitter - 1);
                beam_positions.insert(splitter + 1);
            }
        }
    }
    println!("Total splits: {number_of_splits}");
}