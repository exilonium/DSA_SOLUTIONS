impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        if mat == target {
            return true;
        }

        let mut rotated = mat.clone();
        let size = mat.len();
        for _ in 0..3 {
            let mut buf = rotated.clone();

            for i in 0..size {
                for j in 0..size {
                    rotated[i][j] = buf[size - j - 1][i];
                }
            }

            if target == rotated {
                return true;
            }
        }

        false
    }
}
