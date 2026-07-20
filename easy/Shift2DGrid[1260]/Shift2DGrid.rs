impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let total = rows * cols;
        let k = (k as usize) % total;

        let mut ans = vec![vec![0; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                let idx = i * cols + j;
                let nxt = (idx + k) % total;
                ans[nxt / cols][nxt % cols] = grid[i][j];
            }
        }

        ans
    }
}
