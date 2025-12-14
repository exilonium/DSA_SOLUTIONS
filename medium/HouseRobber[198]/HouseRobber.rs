use std::cmp::max;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut rob1 = 0;
        let mut rob2 = 0;
        // we doing some shenanigans
        // [rob1, rob2, n , n+1 .....]
        // visualizing array like this will help us understanding the approach
        for i in nums {
            let temp = max(i + rob1, rob2);
            rob1 = rob2; // we going ahead
            rob2 = temp; // this is to store the max we have got from the start to this position (i)
        }
        rob2
    }
}
