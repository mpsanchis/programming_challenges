use crate::util::Part;

mod input;
mod validation;
mod types;

use types::{ID, Range};

fn main_1(ranges: Vec<Range>, ids: Vec<ID>) {
    let mut fresh_ids: Vec<ID> = Vec::new();

    for id in ids {
        if ranges.iter().any(|range| range.contains_id(id)) {
            fresh_ids.push(id);
        }
    }
    println!("Fresh IDs: {:?}", fresh_ids);
    println!("Num of fresh IDs: {:?}", fresh_ids.len());
}

fn main_2(ranges: Vec<Range>) {

}

pub fn main(part: &Part) {
    let (ranges, ids) = input::parse_input();

    match part {
        Part::One => main_1(ranges, ids),
        Part::Two => main_2(ranges),
    }
}