// i still dont understand the Solution
// not my Solution i didn't even understood the question well :)
impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let (z, o, l) = (zero as usize, one as usize, limit as usize);
        let modv = 1_000_000_007;

        let mut dp = vec![vec![[0i64; 2]; o + 1]; z + 1];

        for i in 1..=z.min(l) {
            dp[i][0][0] = 1;
        }

        for j in 1..=o.min(l) {
            dp[0][j][1] = 1;
        }

        for i in 1..=z {
            for j in 1..=o {
                let over0 = if i >= l + 1 { dp[i - l - 1][j][1] } else { 0 };

                let over1 = if j >= l + 1 { dp[i][j - l - 1][0] } else { 0 };

                dp[i][j][0] = (dp[i - 1][j][0] + dp[i - 1][j][1] - over0 + modv) % modv;

                dp[i][j][1] = (dp[i][j - 1][0] + dp[i][j - 1][1] - over1 + modv) % modv;
            }
        }

        ((dp[z][o][0] + dp[z][o][1]) % modv) as i32
    }
}
