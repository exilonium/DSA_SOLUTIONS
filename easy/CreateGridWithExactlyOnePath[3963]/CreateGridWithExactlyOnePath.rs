impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let m = m as usize;
        let n = n as usize;

        let mut grid = vec![vec!['#'; n]; m];

        for j in 0..n {
            grid[0][j] = '.';
        }

        for i in 0..m {
            grid[i][n - 1] = '.';
        }

        grid.into_iter()
            .map(|row| row.into_iter().collect())
            .collect()
    }
}
