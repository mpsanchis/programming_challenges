use std::hash::{Hash, Hasher};

pub type SplitterPositions = Vec<usize>;

pub struct Beam {
    pub position: usize,
    pub weight: usize,
}

impl Beam {
    pub fn merge_with(self, other: Option<&Beam>) -> Self {
        match other {
            None => self,
            Some(other_beam) => {
                Beam { position: self.position, weight: self.weight + other_beam.weight }
            }
        }
    }
}

impl Hash for Beam {
    // Use the same criterion as Eq, to be used in a HashSet
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

impl Eq for Beam {}

impl PartialEq for Beam {
    // Two beams are equal if their positions are equal
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}