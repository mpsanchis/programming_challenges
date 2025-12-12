use std::fs;
use crate::util::logger;

use super::types::Point;

#[derive(Debug)]
pub struct InputPoints {
    pub points: Vec<Point>,
    pub rows: u32,
    pub cols: u32,
    pub point_bottom_right_idx: usize,
}

pub fn parse_input() -> InputPoints {
    logger().logn("Reading input...");
    let input = fs::read_to_string(file!().replace(".rs", ".txt"))
        .expect("could not read input");
    
    let mut points: Vec<Point> = Vec::new();
    let mut max_x: u32 = 0;
    let mut max_y: u32 = 0;
    let mut point_bottom_right_idx: usize = 0;
    let mut min_x = input.lines().next().unwrap().split(',').map(|c| c.parse().unwrap()).collect::<Vec<u32>>()[0];
    let mut min_y = input.lines().next().unwrap().split(',').map(|c| c.parse().unwrap()).collect::<Vec<u32>>()[1];
    
    for (idx, line) in input.lines().enumerate() {
        let coordinates: Vec<u32> = line.split(',').map(|c| c.parse().unwrap()).collect();
        let x = coordinates[0];
        let y = coordinates[1];
        points.push(Point {x,y});
        // Store also the mins
        if x < min_x {
            min_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        //
        if x > max_x {
            max_x = x;
            if y > max_y {
                point_bottom_right_idx = idx;
            }
        }
        if y > max_y {
            max_y = y;
            point_bottom_right_idx = idx;
        }
    }
    let input_points = InputPoints { 
        points, 
        rows: max_y + 1, 
        cols: max_x + 1, 
        point_bottom_right_idx 
    };
    logger().logn(&format!("Input parsed: {} x {}", input_points.rows, input_points.cols));
    logger().logn(&format!("Min x = {}\nMin y = {}", min_x, min_y));
    input_points
}
