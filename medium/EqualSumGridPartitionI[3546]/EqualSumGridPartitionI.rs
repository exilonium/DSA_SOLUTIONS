impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        let mut total: i64 = 0;

        for row in &grid {
            for &x in row {
                total += x as i64;
            }
        }

        if total % 2 != 0 {
            return false;
        }

        let target = total / 2;

        // Horizontal cut
        let mut sum = 0i64;

        for i in 0..m - 1 {
            for &x in &grid[i] {
                sum += x as i64;
            }

            if sum == target {
                return true;
            }
        }

        // Vertical cut
        let mut sum = 0i64;

        for j in 0..n - 1 {
            for i in 0..m {
                sum += grid[i][j] as i64;
            }

            if sum == target {
                return true;
            }
        }

        false
    }
}
