impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut last = 1;
        for c in s.chars() {
            match c {
                '1' => {
                    if last == 0 {
                        return false;
                    }
                    last = 1;
                }
                '0' => last = 0,
                _ => {}
            }
        }
        true
    }
}
