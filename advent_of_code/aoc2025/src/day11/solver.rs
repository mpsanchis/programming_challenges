use std::collections::HashMap;
use super::types::{Graph};

pub struct GraphSolver <'a>{
    graph: &'a Graph,
    ways_out_cache: HashMap<String, Option<u32>>,
}

impl<'a> GraphSolver<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self { 
            graph, 
            ways_out_cache: HashMap::new() 
        }
    }
    
    pub fn find_ways_out(&mut self, from_node_id: &str) -> Option<u32> {
        // base: reached out
        if from_node_id == "out" {
            return Some(1);
        }
        
        // if not out:
        // check if this node was already visited
        if let Some(node_visited) = self.ways_out_cache.get(from_node_id) {
            match node_visited {
                // Node visited and calculated: no need to visit further
                Some(ways_out) => return Some(*ways_out),
                // Node visited but not result yet: you might be in a loop...
                None => return None,
            }
        }
        
        // result not in the cache: iterate all the nodes I can reach
        let mut ways_out = 0;
        for node_id in self.graph.get_connected_nodes(from_node_id).unwrap() {
            if let Some(more_ways_out) = self.find_ways_out(node_id) {
                ways_out += more_ways_out;
            }
        }
        self.ways_out_cache.insert(from_node_id.to_string(), Some(ways_out));
        Some(ways_out)
    }
}