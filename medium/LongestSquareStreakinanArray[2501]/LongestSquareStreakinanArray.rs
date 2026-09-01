use std::collections::HashSet;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let set: HashSet<i64> = nums.iter().map(|&x| x as i64).collect();

        let mut ans = -1;

        for &x in &nums {
            let mut curr = x as i64;
            let mut len = 1;

            while set.contains(&(curr * curr)) {
                curr *= curr;
                len += 1;
            }

            if len >= 2 {
                ans = ans.max(len);
            }
        }

        ans
    }
}
