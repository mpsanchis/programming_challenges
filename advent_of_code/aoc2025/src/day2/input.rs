use std::{fs, ops::Range};

fn input_file() -> String {
    let this_file = file!();
    this_file.replace(".rs", ".txt")
}

pub fn parse_input() -> Vec<Range<u64>> {
    let input = fs::read_to_string(input_file()).expect("could not read input");
    let ranges: Vec<Range<u64>> = input
        .split(",")
        .map(|range| {
            let start: u64 = range.split("-").next().unwrap().parse().unwrap();
            let end: u64 = range.split("-").nth(1).unwrap().parse().unwrap();
            Range{ start, end: end + 1 }
    })
    .collect();
    ranges
}
