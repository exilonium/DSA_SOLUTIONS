impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut result = Vec::with_capacity(rows * cols);

        let (mut top, mut bottom, mut left, mut right) = (0, rows - 1, 0, cols - 1);

        while top <= bottom && left <= right {
            // top row
            for c in left..=right {
                result.push(matrix[top][c]);
            }
            top += 1;

            // right column
            for r in top..=bottom {
                result.push(matrix[r][right]);
            }
            if right == 0 { break; }
            right = right.saturating_sub(1);

            // bottom row
            if top <= bottom {
                for c in (left..=right).rev() {
                    result.push(matrix[bottom][c]);
                }
                bottom = bottom.saturating_sub(1);
            }

            // left column
            if left <= right {
                for r in (top..=bottom).rev() {
                    result.push(matrix[r][left]);
                }
                left += 1;
            }
        }

        result
    }
}
