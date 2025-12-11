use super::types::Point;

pub fn calculate_area(p1: &Point, p2: &Point) -> u64 {
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