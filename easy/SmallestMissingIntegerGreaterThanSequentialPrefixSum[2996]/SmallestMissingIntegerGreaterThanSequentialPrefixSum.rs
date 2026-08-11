use std::collections::HashSet;
impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];

        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] + 1 {
                sum += nums[i];
            } else {
                break;
            }
        }

        let set: HashSet<i32> = nums.into_iter().collect();

        let mut ans = sum;
        while set.contains(&ans) {
            ans += 1;
        }

        ans
    }
}
