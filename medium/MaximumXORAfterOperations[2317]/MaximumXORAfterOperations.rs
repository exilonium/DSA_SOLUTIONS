impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in nums {
            res = res | i;
        }
        res
    }
}
