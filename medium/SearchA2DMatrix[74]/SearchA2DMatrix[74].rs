impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let r = matrix.len();
        let column = matrix[0].len();

        let mut i = 0;
        while i < r && matrix[i][column - 1] < target {
            i += 1;
        }
        if i == r || matrix[i][0] > target {
            return false;
        }
        let mut low: i32 = 0;
        let mut high: i32 = column as i32 - 1;
        while low <= high {
            let mut mid = low + (high - low) / 2;
            if matrix[i][mid as usize] == target {
                return true;
            } else if matrix[i][mid as usize] < target {
                low = mid + 1;
            } else {
                high = mid - 1
            }
        }
        return false;
    }
}
