impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut xor = 0;
        for i in &nums {
            xor ^= i;
        }
        let lowest_bit = xor & -xor;
        let mut res = Vec::from([0, 0]);
        for i in nums {
            if (lowest_bit & i) == 0 {
                res[0] ^= i;
            } else {
                res[1] ^= i;
            }
        }
        res
    }
}
