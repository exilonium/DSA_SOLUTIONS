impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let k = k as u32;
        for i in 0..nums.len() {
            if i.count_ones() == k {
                res += nums[i];
            }
        }
        res
    }
}
