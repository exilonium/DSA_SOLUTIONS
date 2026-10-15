use std::collections::HashSet;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;

        for i in 0..n {
            let mut seen = HashSet::new();

            for j in i..n {
                seen.insert(nums[j]);

                let count = seen.len() as i32;
                res += count * count;
            }
        }

        res
    }
}
