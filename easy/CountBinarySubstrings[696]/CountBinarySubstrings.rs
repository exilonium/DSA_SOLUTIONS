impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();

        let mut prev = 0;
        let mut curr = 1;
        let mut ans = 0;

        for i in 1..chars.len() {
            if chars[i] == chars[i - 1] {
                curr += 1;
            } else {
                ans += prev.min(curr);
                prev = curr;
                curr = 1;
            }
        }

        ans + prev.min(curr)
    }
}
