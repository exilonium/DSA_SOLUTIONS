use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        let mut left = 0;
        let mut ans = 0;

        for right in 0..nums.len() {
            let entry = freq.entry(nums[right]).or_insert(0);
            *entry += 1;

            while let Some(&count) = freq.get(&nums[right]) {
                if count <= k {
                    break;
                }

                let x = nums[left];

                if let Some(entry) = freq.get_mut(&x) {
                    *entry -= 1;
                }

                left += 1;
            }

            ans = ans.max(right - left + 1);
        }

        ans as i32
    }
}
