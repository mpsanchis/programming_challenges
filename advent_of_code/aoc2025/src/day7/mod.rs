use std::collections::{HashSet, HashMap};

use crate::util::{Part, logger};

mod input;
mod types;

use types::{Beam, SplitterPositions};

fn main_1(splitters_per_line: Vec<SplitterPositions>) {
    let mut beam_positions: HashSet<usize> = HashSet::new();
    beam_positions.insert(splitters_per_line[0][0]);
    
    let mut number_of_splits = 0;
    for splitters in &splitters_per_line[1..] {
        logger().logn(&format!("Splitters: {:?}", splitters));
        for splitter in splitters {
            if beam_positions.contains(splitter) {
                number_of_splits += 1;
                beam_positions.remove(splitter);
                beam_positions.insert(splitter - 1);
                beam_positions.insert(splitter + 1);
            }
        }
    }
    println!("Total splits: {number_of_splits}");
}

fn main_2(splitters_per_line: Vec<SplitterPositions>) {
    let mut beams: HashMap<
        usize, // key: position of the beam
        usize // value: weight of the beam (how many paths lead to it)
    > = HashMap::new();
    beams.insert(splitters_per_line[0][0], 1);
    
    for splitters in &splitters_per_line[1..] {
        logger().logn(&format!("Splitters: {:?}", splitters));
        for splitter in splitters {
            if let Some(beam) = beams.remove(splitter) {
                let prev_beam = *beams.get(&(splitter - 1)).unwrap_or(&0);
                let next_beam = *beams.get(&(splitter + 1)).unwrap_or(&0);
                
                beams.insert(splitter - 1, beam + prev_beam);
                beams.insert(splitter + 1, beam + next_beam);
            }
        }
    }
    let paths: usize = beams.iter().map(|beam| beam.1).sum();
    println!("Total paths: {paths}");
}

pub fn main(part: &Part) {
    let splitters_per_line = input::parse_input();
    match part {
        Part::One => main_1(splitters_per_line),
        Part::Two => main_2(splitters_per_line),
    }
}