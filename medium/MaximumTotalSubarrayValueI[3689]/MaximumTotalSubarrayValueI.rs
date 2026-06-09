impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let mut mn = nums[0];
        let mut mx = nums[0];

        for &x in &nums {
            mn = mn.min(x);
            mx = mx.max(x);
        }

        (mx as i64 - mn as i64) * k as i64
    }
}
