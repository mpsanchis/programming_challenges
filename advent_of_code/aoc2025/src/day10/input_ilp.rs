use std::fs;

use super::types::{Joltage};
use super::types_ilp::{Coefficients, MinimumPressesProblem};

fn parse_joltages(line: &str) -> Vec<Joltage> {
    line.split(' ')
        .filter(|&section| section.starts_with('{'))
        .take(1)
        .collect::<String>()
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(',')
        .map(|num_str| num_str.parse().expect("Could not parse number {num_str}"))
        .collect()
}

fn parse_coefficients(line: &str, num_joltages: usize) -> Coefficients {
    let switches = line.split(' ')
        .filter(|&section| section.starts_with('('))
        .map(|section| section
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(',')
            .map(|num_str| num_str.parse().expect("Could not parse number {num_str}"))
            .collect::<Vec<usize>>()
        )
        .collect::<Vec<Vec<usize>>>();
    
    let num_switches = switches.len();
    let mut coefficients = vec![vec![]; num_joltages];
    for j in 0..num_joltages {
        let coefficients_row = vec![false; num_switches].iter().enumerate().map(|(i, _)| 
            if switches[i].contains(&j) { true } else { false }
        ).collect::<Vec<bool>>();
        coefficients[j] = coefficients_row;
    }
    
    coefficients
}

pub fn parse_input_ilp() -> Vec<MinimumPressesProblem> {
    let input = fs::read_to_string(file!().replace("_ilp.rs", ".txt"))
        .expect("could not read input");
    
    let mut problems: Vec<MinimumPressesProblem> = Vec::new();
    
    for line in input.lines() {
        let joltages = parse_joltages(line);
        let coefficients = parse_coefficients(line, joltages.len());
        problems.push(MinimumPressesProblem::new(joltages, coefficients));
    }
    problems
}