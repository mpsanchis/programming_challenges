#[derive(Debug)]
pub struct BoxPosition(
    pub u32,
    pub u32,
    pub u32,
);

#[derive(Debug)]
pub struct Box {
    pub position: BoxPosition,
    pub cluster_id: Option<ClusterID>,
}

impl Box {
    pub fn distance_to(&self, other: &Box) -> f64 {
        ((self.position.0 as i128 - other.position.0 as i128).pow(2) +
         (self.position.1 as i128 - other.position.1 as i128).pow(2) +
         (self.position.2 as i128 - other.position.2 as i128).pow(2)) as f64
    }
}

pub struct DistanceBox2Box {
    pub distance: f64,
    pub box1_idx: usize,
    pub box2_idx: usize,
}

pub type ClusterID = u32;