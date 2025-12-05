use super::{Range};
use crate::util::logger;

pub fn reduce_ranges_one_round(ranges: Vec<Range>) -> Vec<Range> {
    let mut reduced_ranges: Vec<Range> = vec![ranges[0]];

    for i in 1..ranges.len() {
        let mut has_been_merged = false;

        for j in 0..reduced_ranges.len() {
            if let Some(range) = Range::merge_ranges(&ranges[i], &reduced_ranges[j]) {
                logger().logn(&format!("Merging {:?} and {:?} into {:?}", &ranges[i], &reduced_ranges[j], &range));
                reduced_ranges[j] = range;
                has_been_merged = true;
            }
        }

        if !has_been_merged {
            logger().logn(&format!("Adding new range {:?}", &ranges[i]));
            reduced_ranges.push(ranges[i].clone());
        }
    }
    reduced_ranges
}

pub fn reduce_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    let mut iter = 0;
    loop {
        let original_size = ranges.len();

        ranges = reduce_ranges_one_round(ranges);

        let new_size = ranges.len();

        let has_been_reduced = new_size < original_size;
        logger().logn(&format!("Iter {}: reduced? {}", iter, has_been_reduced));
        logger().logn(&format!("ranges: {:?}", ranges));
        if !has_been_reduced
        || iter > 5
        {
            break;
        }
        iter += 1;
    }
    ranges
}