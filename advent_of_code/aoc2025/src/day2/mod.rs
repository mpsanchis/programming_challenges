use crate::util::{Part, logger};

mod input;
mod validation;

pub fn main(part: &Part) {
    let logger = logger();
    let validation_fn = match part {
        Part::One => validation::is_invalid_id,
        Part::Two => validation::is_invalid_id_2,
    };
    let input_ranges = input::parse_input();
    let mut invalid_ids: Vec<u64> = Vec::new();

    for range in input_ranges {
        let mut invalid_ids_local: Vec<u64> = Vec::new();
        logger.logn(&format!("Range: {}-{}", range.start, range.end));
        for number in range {
            if validation_fn(number) {
                invalid_ids_local.push(number);
            }
        }
        logger.logn(&format!("Partial invalid IDs: {invalid_ids_local:?}"));
        invalid_ids.extend(invalid_ids_local);
    }
    println!("Total invalid IDs: {invalid_ids:?}");
    println!("Number of invalid IDs: {}", invalid_ids.len());
    println!("sum of invalid IDs: {}", invalid_ids.iter().sum::<u64>());
}
