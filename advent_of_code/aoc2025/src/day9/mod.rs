use crate::util::Part;

mod input;
mod calculation;
mod types;

use types::Point;

fn part_one(points: Vec<Point>) {
    let mut areas = calculation::calculate_areas(&points);
    println!("Largest area: {}", areas.pop().unwrap());
}

fn part_two(points: Vec<Point>) {
    
}

pub fn main(part: &Part) {
    let points = input::parse_input();
    match part {
        Part::One => part_one(points),
        Part::Two => part_two(points),
    }
}