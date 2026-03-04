use std::collections::HashSet;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            let mut row = HashSet::new();
            let mut column = HashSet::new();
            for j in 0..9 {
                let elem = board[i][j];
                let relem = board[j][i];
                if row.contains(&elem) || column.contains(&relem) {
                    return false;
                }
                if elem != '.' {
                    row.insert(elem);
                }
                if relem != '.' {
                    column.insert(relem);
                }
            }
        }
        for rjump in 0..3 {
            for cjump in 0..3 {
                let mut mybox = HashSet::new();
                for i in 0..3 {
                    for j in 0..3 {
                        let elem = board[i + 3 * rjump][j + 3 * cjump];
                        if elem != '.' {
                            if mybox.contains(&elem) {
                                return false;
                            }
                            mybox.insert(elem);
                        }
                    }
                }
            }
        }
        true
    }
}
