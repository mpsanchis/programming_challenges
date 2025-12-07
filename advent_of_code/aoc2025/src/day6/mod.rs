use crate::util::{logger, Part};

mod input;
mod calculation;
mod types;

use types::Operation;

fn main_1() {
    let columns = input::parse_input_1();
    let mut results: Vec<u64> = Vec::new();

    for column in columns {
        let result = match column.operation {
            Operation::Add => column.numbers.iter().fold(0, |acc, e| {acc + e}),
            Operation::Multiply => column.numbers.iter().fold(1, |acc, e| {acc * e}),
        };
        results.push(result);
    }
    println!("Results: {:?}", results);
    println!("Final result: {}", results.iter().sum::<u64>());
}

fn main_2() {
    let problems = input::parse_input_2();

    let mut results: Vec<u64> = Vec::new();

    for problem in problems {
        problem.pretty_print();
        let result = match problem.operation {
            Operation::Add => problem.numbers.iter()
                .map(|v| {
                    let digits = v.len();
                    v.iter().enumerate().fold(0, |acc, (i, digit)| acc + digit * 10u64.pow((digits - i - 1) as u32))
                })
                .fold(0, |acc, e| {acc + e}),
            Operation::Multiply => problem.numbers.iter()
                .map(|v| {
                    let digits = v.len();
                    v.iter().enumerate().fold(0, |acc, (i, digit)| acc + digit * 10u64.pow((digits - i - 1) as u32))
                })
                .fold(1, |acc, e| {acc * e}),
        };
        results.push(result);
    }
    println!("Results: {:?}", results);
    println!("Final result: {}", results.iter().sum::<u64>());
}

pub fn main(part: &Part) {
    match part {
        Part::One => main_1(),
        Part::Two => main_2(),
    }
}