impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut open = 0;
        let mut unmatched = 0;

        for ch in s.chars() {
            if ch == '(' {
                open += 1;
            } else {
                if open > 0 {
                    open -= 1;
                } else {
                    unmatched += 1;
                }
            }
        }

        unmatched + open
    }
}
