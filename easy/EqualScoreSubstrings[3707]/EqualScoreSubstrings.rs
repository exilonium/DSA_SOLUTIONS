impl Solution {
    pub fn score_balance(s: String) -> bool {
        let mut right = 0;

        for c in s.bytes() {
            right += (c - b'a' + 1) as i32;
        }

        let mut left = 0;

        for c in s.bytes().take(s.len() - 1) {
            let score = (c - b'a' + 1) as i32;

            left += score;
            right -= score;

            if left == right {
                return true;
            }
        }

        false
    }
}
