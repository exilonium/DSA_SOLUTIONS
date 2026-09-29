impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const MOD: i64 = 12345;

        let m = grid.len();
        let n = grid[0].len();

        let mut ans = vec![vec![0; n]; m];

        let mut prefix = 1i64;

        for i in 0..m {
            for j in 0..n {
                ans[i][j] = prefix as i32;
                prefix = prefix * grid[i][j] as i64 % MOD;
            }
        }

        let mut suffix = 1i64;

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                ans[i][j] = (ans[i][j] as i64 * suffix % MOD) as i32;
                suffix = suffix * grid[i][j] as i64 % MOD;
            }
        }

        ans
    }
}
