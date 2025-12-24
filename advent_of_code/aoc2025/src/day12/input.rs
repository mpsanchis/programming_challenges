use std::fs;

use super::types::{Region};

fn parse_region(region_size: &str) -> Region {
    let mut wl = region_size.split('x');
    
    let w: u16 = wl.next().unwrap().parse().unwrap();
    let l: u16 = wl.next().unwrap().parse().unwrap();
    
    Region { width: w, length: l }
}

pub fn parse_input() -> Vec<(Region, Vec<u16>)> {
    let input = fs::read_to_string(file!().replace(".rs", ".txt"))
        .expect("could not read input");
    
    let mut parsed_inputs = Vec::new();
    
    for line in input.lines() {
        if !line.contains("x") {
            continue;
        }
        
        let line_data: Vec<String> = line.split(' ').map(|node_id| 
            node_id.trim_end_matches(":").to_string()
        ).collect();
        
        let region = parse_region(&line_data[0]);
        let presents: Vec<u16> = (&line_data[1..]).iter().map(|p| p.parse().unwrap()).collect();
        
        parsed_inputs.push((region, presents));
    }
    parsed_inputs
}