use super::types::{Box, DistanceBox2Box};

pub fn calculate_distances(boxes: &Vec<Box>) -> Vec<DistanceBox2Box> {
    let mut distances: Vec<DistanceBox2Box> = Vec::new();
    for (i, b_i) in boxes.iter().enumerate() {
        for j in 0..i {
            let b_j = &boxes[j];
            if i != j {
                let distance = b_i.distance_to(b_j);
                let distance = DistanceBox2Box {
                    box1_idx: i,
                    box2_idx: j,
                    distance,
                };
                distances.push(distance);
            }
        }
    }
    distances.sort_by(|d1, d2| d1.distance.total_cmp(&d2.distance));
    distances
}