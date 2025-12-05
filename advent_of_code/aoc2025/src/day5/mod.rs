use crate::util::{logger, Part};

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
    let mut total_fresh_ids = 0;

    let reduced_ranges = validation::reduce_ranges(ranges);

    for range in &reduced_ranges {
        total_fresh_ids += range.size();
    }

    logger().logn(&format!("Reduced ranges: {:?}", &reduced_ranges));
    println!("Total fresh IDs: {total_fresh_ids}");
}

pub fn main(part: &Part) {
    let (ranges, ids) = input::parse_input();

    match part {
        Part::One => main_1(ranges, ids),
        Part::Two => main_2(ranges),
    }
}