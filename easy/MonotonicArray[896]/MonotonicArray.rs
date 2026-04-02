impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let size = nums.len();
        let mut inc = true;
        let mut dec = true;

        for i in 1..size {
            if nums[i] > nums[i - 1] {
                dec = false;
            } else if nums[i] < nums[i - 1] {
                inc = false;
            }
            if !inc && !dec {
                return false;
            }
        }
        true
    }
}
