use std::collections::HashMap;

pub struct CompositionCalculator {
    cache: HashMap<(u32, usize), Vec<Vec<u32>>>,
}

fn generate_compositions(
    pos: usize,
    k: usize,
    remaining: u32,
    current: &mut Vec<u32>,
    result: &mut Vec<Vec<u32>>
) {
    // Base case: array is complete
    if pos == k - 1 {
        current[pos] = remaining;
        result.push(current.clone());
        return;
    }

    // Choose value for current position
    for x in 0..=remaining {
        current[pos] = x;
        generate_compositions(
            pos + 1,
            k,
            remaining - x,
            current,
            result
        );
    }
}

impl CompositionCalculator {
    pub fn new() -> Self {
        CompositionCalculator {
            cache: HashMap::new(),
        }
    }

    pub fn sums_of_n_in_k_parts(&mut self, n: u32, k: usize) -> Vec<Vec<u32>> {
        if let Some(result) = self.cache.get(&(n, k)) {
            return result.clone();
        }
        
        let mut result = Vec::new();
        let mut current = vec![0; k];
        generate_compositions(0, k, n, &mut current, &mut result);
        
        self.cache.insert((n, k), result.clone());
        result 
    }
}