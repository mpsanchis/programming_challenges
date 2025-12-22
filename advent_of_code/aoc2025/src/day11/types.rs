use std::collections::HashMap;

pub type NodeId = String;

pub struct Graph(HashMap<NodeId, Vec<NodeId>>);

impl Graph {
    pub fn new() -> Self {
        Graph(HashMap::new())
    }
    pub fn insert(&mut self, node_id: NodeId, connected_nodes: Vec<NodeId>) {
        self.0.insert(node_id, connected_nodes);
    }
    pub fn get_connected_nodes(&self, node_id: &str) -> Option<&Vec<NodeId>> {
        self.0.get(node_id)
    }
}