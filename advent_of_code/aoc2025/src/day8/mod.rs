use std::collections::{HashMap, HashSet};

use crate::{day8::types::DistanceBox2Box, util::{Part, logger}};

mod types;
mod input;
mod calculation;

use types::{Box, ClusterID};

fn merge_clusters(
    box1_idx: usize,
    box2_idx: usize,
    boxes: &mut Vec<Box>,
    boxes_per_cluster: &mut HashMap<ClusterID, HashSet<usize>>,
) {
    logger().logn("=====");
    logger().logn(&format!("Merging boxes [{box1_idx}] {:?} and [{box2_idx}] {:?}", &boxes[box1_idx], &boxes[box2_idx]));
    let (from_box_idx, to_cluster_id) = match boxes[box1_idx].cluster_id {
        Some(cluster_id) => (box2_idx, cluster_id),
        None => {
            match boxes[box2_idx].cluster_id {
                Some(cluster_id) => (box1_idx, cluster_id),
                None => {
                    // None of the box belongs to a cluster: we need to create one (starting from Key 1)
                    let new_cluster_id = boxes_per_cluster.keys().map(|k| k + 1).max().unwrap_or(1);
                    let mut new_cluster = HashSet::new();
                    new_cluster.insert(box1_idx);
                    new_cluster.insert(box2_idx);
                    boxes_per_cluster.insert(new_cluster_id, new_cluster);
                    // And we can already point the boxes to that cluster
                    boxes[box1_idx].cluster_id = Some(new_cluster_id);
                    boxes[box2_idx].cluster_id = Some(new_cluster_id);
                    logger().logn(&format!("Created new cluster [{}]: {:?}", new_cluster_id, boxes_per_cluster.get(&new_cluster_id).unwrap()));
                    (box1_idx, new_cluster_id)
                }
            }
        }
    };
    
    // Add all boxes from 'from_cluster' into 'to_cluster'
    let from_box = &boxes[from_box_idx];
    let from_cluster_id = from_box.cluster_id.unwrap_or(0); // ID 0 is never used for a cluster
    
    if from_cluster_id == to_cluster_id {
        logger().logn("Boxes already in same cluster. Nothing to do.");
        logger().logn("=====");
        // Handle the case where the boxes are already connected: nothing to do
        return;
    }
    
    let from_cluster = boxes_per_cluster.remove(&from_cluster_id).unwrap_or_else(|| {
        let mut hash_set = HashSet::new();
        hash_set.insert(from_box_idx);
        hash_set
    });
    // Add all boxes from 'from_cluster' into 'to_cluster'
    boxes_per_cluster.get_mut(&to_cluster_id).unwrap().extend(&from_cluster);

    // Update also the boxes themselves, to point to their new cluster ID
    for box_idx in from_cluster {
        boxes[box_idx].cluster_id = Some(to_cluster_id);
    }
    logger().logn(&format!("Boxes merged into cluster [{to_cluster_id}] {:?}", boxes_per_cluster.get(&to_cluster_id).unwrap()));
    logger().logn("=====");
}

fn main_1(mut boxes: Vec<Box>) {
    // Implementation for part 1
    let distances = calculation::calculate_distances(&boxes);
    
    let mut boxes_per_cluster: HashMap<ClusterID, HashSet<usize>> = HashMap::new();
    
    let max_iter = match boxes.len() {
        20 => 10, // simple input
        _ => 1000, // actual input
    };
    
    for dist in &distances[0..max_iter] {
        merge_clusters(
            dist.box1_idx, 
            dist.box2_idx, 
            &mut boxes, 
            &mut boxes_per_cluster
        );
    }
    
    let mut clusters_per_length = boxes_per_cluster.iter().map(|cluster| cluster.1.len() as u64).collect::<Vec<u64>>();
    clusters_per_length.sort_by(|a, b| b.cmp(a)); // reverse order: from big to small
    let three_biggest_clusters = [clusters_per_length[0], clusters_per_length[1], clusters_per_length[2]];
    println!("Three biggest clusters sizes: {:?}", three_biggest_clusters);
    println!("Result: {}", three_biggest_clusters[0] * three_biggest_clusters[1] * three_biggest_clusters[2]);
}

fn main_2(box_positions: Vec<Box>) {
    // Implementation for part 2
}

pub fn main(part: &Part) {
    let box_positions = input::parse_input();
    
    match part {
        Part::One => main_1(box_positions),
        Part::Two => main_2(box_positions),
    }
}