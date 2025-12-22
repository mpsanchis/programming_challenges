use std::fs;

use super::types::{Graph};

pub fn parse_input() -> Graph {
    let input = fs::read_to_string(file!().replace(".rs", ".txt"))
        .expect("could not read input");
    
    let mut graph = Graph::new();
    
    for line in input.lines() {
        let node_ids: Vec<String> = line.split(' ').map(|node_id| 
            node_id.trim_end_matches(":").to_string()
        ).collect();
        
        let node_id_from = &node_ids[0];
        let node_ids_to = &node_ids[1..];
        
        graph.insert(node_id_from.clone(), Vec::from(node_ids_to));
    }
    graph
}