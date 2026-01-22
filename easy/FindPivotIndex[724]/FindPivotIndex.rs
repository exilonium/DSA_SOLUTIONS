impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut left = 0;
        for i in 0..nums.len() {
            if left == sum - left - nums[i] {
                return i as _;
            }
            left += nums[i];
        }
        -1
    }
}
