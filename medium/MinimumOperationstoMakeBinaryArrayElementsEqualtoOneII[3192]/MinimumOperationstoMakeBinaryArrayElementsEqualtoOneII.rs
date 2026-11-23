impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut flips = 0;

        for i in 0..nums.len() {
            if nums[i] == (flips % 2) {
                flips += 1;
            }
        }

        flips
    }
}
