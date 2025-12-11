use std::fs;
use super::types::Point;

pub fn parse_input() -> Vec<Point> {
    let input = fs::read_to_string(file!().replace(".rs", ".txt"))
        .expect("could not read input");
    
    let mut points: Vec<Point> = Vec::new();
    
    for line in input.lines() {
        let coordinates: Vec<u32> = line.split(',').map(|c| c.parse().unwrap()).collect();
        points.push(Point {
            x: coordinates[0],
            y: coordinates[1],
        });
    }
    points
}
