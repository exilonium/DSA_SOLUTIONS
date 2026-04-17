impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut curr = 0;
        let mut sum = 0;
        for i in 2..nums.len() {
            if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
                curr += 1;
                sum += curr;
            } else {
                curr = 0;
            }
        }
        sum
    }
}
