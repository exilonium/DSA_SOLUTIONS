impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut total: i64 = 0;
        let mut ncount = 0;
        let mut min = i32::MAX;
        for row in matrix {
            for i in row {
                total += i.abs() as i64;
                if i < 0 {
                    ncount += 1;
                }
                min = min.min(i.abs());
            }
        }
        if ncount % 2 == 1 {
            total -= 2 * min as i64;
        }
        total
    }
}
