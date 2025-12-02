mod input;
mod validation;
mod util;

pub fn main() {
    let input_ranges = input::parse_input();
    let mut invalid_ids: Vec<u64> = Vec::new();

    for range in input_ranges {
        let mut invalid_ids_local: Vec<u64> = Vec::new();
        println!("Range: {}-{}", range.start, range.end);
        for number in range {
            if validation::is_invalid_id_2(number) {
                invalid_ids_local.push(number);
            }
        }
        println!("Partial invalid IDs: {invalid_ids_local:?}");
        invalid_ids.extend(invalid_ids_local);
    }
    println!("Total invalid IDs: {invalid_ids:?}");
    println!("sum of invalid IDs: {}", invalid_ids.iter().sum::<u64>());
}
