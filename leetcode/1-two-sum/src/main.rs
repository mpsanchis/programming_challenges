struct Solution();

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_sorted = nums.clone();
        nums_sorted.sort();

        let mut solution: Vec<i32> = vec![0; 2];
        let mut i1 = 0; // start
        let mut i2 = nums.len() - 1; // end
        let mut current = nums_sorted[i1] + nums_sorted[i2];

        while current != target {
           if current < target {
                i1 += 1;
            } else {
                i2 -= 1;
            }
            current = nums_sorted[i1] + nums_sorted[i2];
        }

        if current != target {
            panic!("No solution found for input: {:?}", nums);
        }

        solution[0] = nums.iter().enumerate().find(|&(_,&num)| num == nums_sorted[i1]).map(|(idx, _)| idx).unwrap() as i32;
        solution[1] = nums.iter().enumerate().find(|&(idx,&num)| {
            num == nums_sorted[i2] && (idx as i32) != solution[0]
        }).map(|(idx, _)| idx).unwrap() as i32;
        solution
    }
}

fn main() {

    println!("S1: {:?}", Solution::two_sum(vec![2,7,11,15], 9));
    println!("S2: {:?}", Solution::two_sum(vec![3,2,4], 6));
    println!("S2: {:?}", Solution::two_sum(vec![3,3], 6));
}
