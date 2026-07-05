impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut foundit = false;
        let mut count = 0;
        for c in s.chars() {
            match c {
                '|' => {
                    foundit = !foundit;
                }
                '*' => {
                    if !foundit {
                        count += 1;
                    }
                }
                _ => {}
            }
        }
        count
    }
}
