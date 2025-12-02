mod input;
mod validation;

pub fn main() {
    let input_ranges = input::parse_input();
    let mut invalid_ids: Vec<u64> = Vec::new();

    for range in input_ranges {
        for number in range {
            if validation::is_invalid_id(number) {
                invalid_ids.push(number);
            }
        }
    }
    println!("invalid IDs: {invalid_ids:?}");
    println!("sum of invalid IDs: {}", invalid_ids.iter().sum::<u64>());
}
