impl Solution {
    pub fn count_rotations(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();

        let mut score = 0i32;

        for i in 0..n - 1 {
            score += (s[i] == s[i + 1]) as i32;
        }

        let mut ans = (score == k) as i32;

        for i in 0..n - 1 {
            // Remove the old first adjacent pair.
            score -= (s[i] == s[i + 1]) as i32;

            // Add the new adjacent pair at the end.
            score += (s[i] == s[(i + n - 1) % n]) as i32;

            ans += (score == k) as i32;
        }

        ans
    }
}
