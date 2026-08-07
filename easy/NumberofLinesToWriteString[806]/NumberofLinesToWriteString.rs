impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut lines = 1;
        let mut extra = 100;
        for c in s.chars() {
            let idx = c as u8 - b'a'; // give the char value
            let i = widths[idx as usize];
            if extra - i >= 0 {
                extra -= i;
            } else {
                lines += 1;
                extra = 100 - i;
            }
        }
        vec![lines, 100 - extra]
    }
}
