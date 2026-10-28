impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut dp = vec![vec![[false; 1024]; n]; m];
        dp[0][0][grid[0][0] as usize] = true;

        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                }

                let val = grid[i][j] as usize;

                for x in 0..1024 {
                    if i > 0 && dp[i - 1][j][x] {
                        dp[i][j][x ^ val] = true;
                    }

                    if j > 0 && dp[i][j - 1][x] {
                        dp[i][j][x ^ val] = true;
                    }
                }
            }
        }

        dp[m - 1][n - 1]
            .iter()
            .position(|&reachable| reachable)
            .unwrap_or(0) as i32
    }
}
