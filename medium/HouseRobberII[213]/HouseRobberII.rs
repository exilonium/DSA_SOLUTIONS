// this is the reused code from the HouseRobber I
// wer're just doing the search excluding both start and end element once
// and special case where array is only one element so we returning that index
// cause who's stopping me from robbing a single guy :)
use std::cmp::max;
impl Solution {
    fn helper(nums: &[i32]) -> i32 {
        let mut rob1 = 0;
        let mut rob2 = 0;
        for i in 0..nums.len() {
            let temp = max(rob1 + nums[i], rob2);
            rob1 = rob2;
            rob2 = temp;
        }
        rob2
    }
    pub fn rob(nums: Vec<i32>) -> i32 {
        let size = nums.len();
        if size < 2 {
            return nums[0];
        } else {
            return max(Self::helper(&nums[1..]), Self::helper(&nums[..size - 1]));
        }
    }
}
