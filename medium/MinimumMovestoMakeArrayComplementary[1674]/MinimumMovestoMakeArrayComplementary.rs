impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        let mut diff = vec![0; (2 * limit + 2) as usize];

        for i in 0..n / 2 {
            let a = nums[i];
            let b = nums[n - 1 - i];

            let low = 1 + a.min(b);
            let high = limit + a.max(b);
            let sum = a + b;

            // Default: 2 moves for all sums in [2, 2*limit]
            diff[2] += 2;
            diff[(2 * limit + 1) as usize] -= 2;

            // 1 move for [low, high]
            diff[low as usize] -= 1;
            diff[(high + 1) as usize] += 1;

            // 0 moves for sum
            diff[sum as usize] -= 1;
            diff[(sum + 1) as usize] += 1;
        }

        let mut ans = i32::MAX;
        let mut current = 0;

        for s in 2..=2 * limit {
            current += diff[s as usize];
            ans = ans.min(current);
        }

        ans
    }
}
