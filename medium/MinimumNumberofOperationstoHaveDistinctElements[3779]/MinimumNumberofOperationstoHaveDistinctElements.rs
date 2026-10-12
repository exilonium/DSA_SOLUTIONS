use std::collections::HashSet;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::new();

        for i in (0..nums.len()).rev() {
            if !seen.insert(nums[i]) {
                return (i / 3 + 1) as i32;
            }
        }

        0
    }
}
