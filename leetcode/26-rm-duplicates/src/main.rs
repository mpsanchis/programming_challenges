struct Solution;

impl Solution {
    fn next_different_num(nums: &[i32], curr: usize) -> Option<usize> {
        let curr_val = nums[curr];

        for j in curr..(nums.len()) {
            let next_val = nums[j];
            if next_val != curr_val {
                return Some(j);
            }
        }
        None
    }
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut diff_vals = 1;

        while let Some(j_next) = Solution::next_different_num(nums, j) {
            nums[i+1] = nums[j_next];
            diff_vals += 1;
            i += 1;
            j = j_next;
        }

        diff_vals
    }
}

fn main() {
    let mut v = vec![0,0,1,1,1,2,2,3,3,4];
    let k = Solution::remove_duplicates(&mut v);

    println!("k: {}", k);
    println!("v: {:?}", v);
}
