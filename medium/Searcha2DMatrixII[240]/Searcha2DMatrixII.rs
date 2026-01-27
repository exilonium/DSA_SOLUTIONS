impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (r, c) = (matrix.len(), matrix[0].len());
        let mut row = 0;
        let mut col = c - 1;
        let mut elem = matrix[row][col];
        while row < r && col < c {
            // here im using col< c cause of integer underflow of col and col>=0
            // is just useless in this case we can typecast to i32 to mitigate this but col<c is a
            // simple guard since if col becomes zero it will pass but on the next decement it will
            // underflow and do this error the len is 1 but the index is 18446744073709551615
            elem = matrix[row][col];
            if elem > target {
                col -= 1;
            } else if elem < target {
                row += 1;
            } else {
                return true;
            }
        }
        false
    }
}
