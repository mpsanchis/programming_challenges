use std::fs;

fn input_file() -> String {
    let this_file = file!();
    this_file.replace(".rs", ".txt")
}

pub fn parse_input() -> Vec<Vec<u8>> {
    let input = fs::read_to_string(input_file()).expect("could not read input");
    let banks: Vec<Vec<u8>> = input
        .lines()
        .map(|bank_str| bank_str
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        )
    .collect();
    banks
}