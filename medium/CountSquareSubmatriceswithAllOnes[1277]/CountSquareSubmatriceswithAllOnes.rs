impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut matrix = matrix;
        let m = matrix.len();
        let n = matrix[0].len();

        let mut ans: i32 = 0;
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 1 {
                    matrix[i][j] +=
                        matrix[i][j - 1].min(matrix[i - 1][j - 1].min(matrix[i - 1][j]));
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                ans += matrix[i][j];
            }
        }
        ans
    }
}
