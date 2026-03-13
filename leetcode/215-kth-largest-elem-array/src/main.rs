use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Element(i32);

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut first_k: BinaryHeap<Element> = BinaryHeap::with_capacity(k as usize);
        // Fill the first k positions
        for i in 0..k {
            first_k.push(Element(nums[i as usize]));
        }
        for num in &nums[(k as usize)..] {
            // Only add the element if it is greater than the current kth
            if *num > first_k.peek().unwrap().0 {
                first_k.push(Element(*num));
                first_k.pop();
            }
        }

        first_k.pop().unwrap().0
    }
}

fn main() {
    let nums = vec![3,2,1,5,6,4];
    let k = 2;
    println!("Solution for nums={:?}, k={k}: \t{}", nums, Solution::find_kth_largest(nums.clone(), k));
    let nums = vec![3,2,3,1,2,4,5,5,6];
    let k = 4;
    println!("Solution for nums={:?}, k={k}: \t{}", nums, Solution::find_kth_largest(nums.clone(), k));
}
