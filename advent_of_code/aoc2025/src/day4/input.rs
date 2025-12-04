use std::fs;

pub fn parse_input() -> Vec<Vec<char>> {
    let input = fs::read_to_string(file!().replace(".rs", ".txt")).expect("could not read input");

    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}
