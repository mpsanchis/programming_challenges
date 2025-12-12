use super::types::{Grid, Point};

fn calculate_area(p1: &Point, p2: &Point) -> u64 {
    let base = (p2.x as i64 - p1.x as i64).abs() + 1;
    let height = (p2.y as i64 - p1.y as i64).abs() + 1;
    (base * height) as u64
}

pub fn calculate_areas(points: &Vec<Point>) -> Vec<u64> {
    let mut areas: Vec<u64> = Vec::new();
    
    for i in 0..points.len() {
        for j in (i+1)..points.len() {
            areas.push(calculate_area(&points[i], &points[j]));
        }
    }
    areas.sort();
    areas
}

pub fn calculate_biggest_area_in_grid(grid: &Grid, points: &[Point]) -> u64 {
    let mut biggest_area = 0;
    for i in 0..points.len() {
        for j in (i+1)..points.len() {
            let area = calculate_area(&points[i], &points[j]);
            if (area > biggest_area) && (grid.rectangle_is_inside(&points[i], &points[j])) {
                biggest_area = area;
            }
        }
    }
    
    biggest_area
}