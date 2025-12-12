use crate::util::{Part, logger};

mod input;
mod calculation;
mod types;

use types::{Grid, Point};
use input::InputPoints;
use calculation::{calculate_areas, calculate_biggest_area_in_grid};

fn part_one(points: Vec<Point>) {
    let mut areas = calculate_areas(&points);
    println!("Largest area: {}", areas.pop().unwrap());
}

fn part_two(input_points: InputPoints) {
    logger().logn("Starting part two...");
    let InputPoints { points, rows, cols, point_bottom_right_idx } = input_points;
    logger().logn("Creating empty grid...");
    panic!("Hold on");
    let mut grid = Grid::new(rows, cols);
    // paint walls green
    logger().logn("Connecting points...");
    grid.connect_points_green(&points);
    logger().logn(&format!("After connecting points, grid is:\n{}", &grid));
    // choose (hopefully) inner point
    let point_bottom_right = &points[point_bottom_right_idx];
    let (x_start, y_start) = (point_bottom_right.x - 1, point_bottom_right.y - 1);
    // paint inside green
    println!("Painting inside green...");
    grid.paint_inside_green(x_start, y_start);
    // calculate areas, and keep max only if valid square
    let biggest_area = calculate_biggest_area_in_grid(&grid, &points);
    println!("Biggest area: {}", biggest_area);
}

pub fn main(part: &Part) {
    let input_points = input::parse_input();
    match part {
        Part::One => part_one(input_points.points),
        Part::Two => part_two(input_points),
    }
}