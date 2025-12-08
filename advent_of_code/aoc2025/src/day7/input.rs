use std::fs;

use super::types::SplitterPositions;

pub fn parse_input() -> Vec<SplitterPositions> {
    let input = fs::read_to_string(file!().replace(".rs", ".txt"))
        .expect("could not read input");
    
    let mut splitters_per_line: Vec<SplitterPositions> = Vec::new(); //vec![Vec::new(); input.lines().count()];
    
    for line in input.lines() {
        let mut splitter_positions: SplitterPositions = Vec::new();
        for (i, c) in line.char_indices() {
            if c == 'S' || c == '^' {
                splitter_positions.push(i);
            }
        }
        splitters_per_line.push(splitter_positions);
    }
    splitters_per_line
}