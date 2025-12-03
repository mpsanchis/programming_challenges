use crate::util::{Part, logger};

mod input;
mod optimisation;

pub fn main(part: &Part) {
    let mut joltages: Vec<u16> = Vec::new();

    let banks = input::parse_input();
    for bank in banks {
        let max_joltage = optimisation::max_joltage(&bank);
        joltages.push(max_joltage);
    }

    println!("Max joltages: {:?}", joltages);
}