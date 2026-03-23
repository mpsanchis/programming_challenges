struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut i_result = -1;
        let mut i_left: usize = 0;
        let mut i_right: usize = nums.len();

        loop {
            let i_mid: usize = (i_right + i_left) / 2;
            if nums[i_mid] == target {
                i_result = i_mid as i32;
                break;
            } else if nums[i_mid] > target {
                if i_right != i_mid {
                    // Happy path
                    i_right = i_mid;
                } else {
                    // No update possible: would result in endless loop
                    break;
                }
            } else { // nums[i_mid] < target
                if i_left != i_mid {
                    i_left = i_mid;
                } else {
                    break;
                }
            }
        }
        i_result
    }
}

fn main() {
    let nums = vec![-1,0,3,5,9,12];
    let res = Solution::search(nums, 9);
    assert_eq!(res, 4);

    let nums = vec![-1,0,3,5,9,12];
    let res = Solution::search(nums, 2);
    assert_eq!(res, -1);

    let nums = vec![-1,0];
    let res = Solution::search(nums, 0);
    assert_eq!(res, 1);

    let nums = vec![-1,0];
    let res = Solution::search(nums, -1);
    assert_eq!(res, 0);
}
