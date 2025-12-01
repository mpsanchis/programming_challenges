use std::fs;

pub fn parse_input() -> Vec<i16> {
    let input = fs::read_to_string("src/day1/input.txt").expect("could not read input");
    let rotations: Vec<i16> = input.lines().map(|line| {
        let dir_str = line.chars().nth(0).unwrap();
        let dir: i16 = match dir_str {
            'L' => -1,
            'R' => 1,
            _ => panic!("invalid direction"),
        };
        let magnitude: i16 = line[1..].parse().expect("invalid distance");
        if dir > 0 {
            magnitude
        } else {
            -magnitude
        }
    })
    .collect();
    rotations
}
