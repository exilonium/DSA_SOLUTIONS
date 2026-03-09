// not my Solution i didn't even understood the question well :)
impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let m = zero as usize;
        let n = one as usize;
        let limit = limit as usize;
        let modv: i64 = 1_000_000_007;

        let mut dp = vec![vec![[0i64; 2]; n + 1]; m + 1];

        for i in 1..=m.min(limit) {
            dp[i][0][0] = 1;
        }
        for j in 1..=n.min(limit) {
            dp[0][j][1] = 1;
        }

        for i in 1..=m {
            for j in 1..=n {
                let over0 = if i > limit {
                    dp[i - limit - 1][j][1]
                } else {
                    0
                };
                let over1 = if j > limit {
                    dp[i][j - limit - 1][0]
                } else {
                    0
                };

                dp[i][j][0] = (dp[i - 1][j][0] + dp[i - 1][j][1] - over0 + modv) % modv;
                dp[i][j][1] = (dp[i][j - 1][0] + dp[i][j - 1][1] - over1 + modv) % modv;
            }
        }

        ((dp[m][n][0] + dp[m][n][1]) % modv) as i32
    }
}
