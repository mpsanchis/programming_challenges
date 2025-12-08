use std::fs;

use super::types::{Box, BoxPosition};

pub fn parse_input() -> Vec<Box> {
    let input = fs::read_to_string(file!().replace(".rs", ".txt"))
        .expect("could not read input");
    
    let mut box_positions: Vec<Box> = Vec::new();
    
    for line in input.lines() {
        let coordinates: Vec<u32> = line.split(',').map(|c| c.parse().unwrap()).collect();
        box_positions.push(Box {
            position: BoxPosition(coordinates[0], coordinates[1], coordinates[2]),
            cluster_id: None,
        });
    }
    box_positions
}
