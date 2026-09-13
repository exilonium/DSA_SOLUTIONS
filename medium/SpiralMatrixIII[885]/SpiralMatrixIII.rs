impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity((rows * cols) as usize);

        let mut r = r_start;
        let mut c = c_start;

        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut step = 1;
        let mut dir = 0;

        ans.push(vec![r, c]);

        while ans.len() < (rows * cols) as usize {
            for _ in 0..2 {
                let (dr, dc) = dirs[dir];

                for _ in 0..step {
                    r += dr;
                    c += dc;

                    if r >= 0 && r < rows && c >= 0 && c < cols {
                        ans.push(vec![r, c]);
                    }
                }

                dir = (dir + 1) % 4;
            }

            step += 1;
        }

        ans
    }
}
