// this is the first Solution i came with, looking at the testcase
// but it fails at [2,1,1,2]
// this code wont run
use std::cmp::max;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut steps = 0;
        let mut sum1 = 0;
        let mut sum2 = 0;
        for i in nums {
            if steps % 2 == 0 {
                sum1 += i;
            } else {
                sum2 += i;
            }
            steps += 1
        }
        return max(sum1, sum2);
    }
}
