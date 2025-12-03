use crate::util::{Part, logger};

mod input;
mod optimisation;

pub fn main(part: &Part) {
    let mut joltages: Vec<u64> = Vec::new();

    let banks = input::parse_input();

    for bank in banks {
        let max_joltage = match part {
            Part::One => optimisation::max_joltage_2(&bank) as u64,
            Part::Two => optimisation::max_joltage_rec(&bank, 11),
        };
        joltages.push(max_joltage);
    }

    println!("Max joltages: {:?}", joltages);
    println!("Sum of max joltages: {:?}", joltages.iter().sum::<u64>());
}