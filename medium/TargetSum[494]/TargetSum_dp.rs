impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        let s = sum + target;
        if s < 0 || s % 2 != 0 {
            return 0;
        }
        let capacity = (s / 2) as usize;
        let mut dp = vec![0; capacity + 1];
        dp[0] = 1;
        for num in nums {
            let num = num as usize;
            for j in (num..=capacity).rev() {
                dp[j] += dp[j - num];
            }
        }
        dp[capacity]
    }
}
