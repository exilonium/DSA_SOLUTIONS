impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() {
            let mut count = 0;
            let mut idx = i;
            while idx != 0 {
                if idx & 1 == 1 {
                    count += 1;
                }
                idx = idx >> 1;
            }
            if count == k {
                res += nums[i];
            }
        }
        res
    }
}
