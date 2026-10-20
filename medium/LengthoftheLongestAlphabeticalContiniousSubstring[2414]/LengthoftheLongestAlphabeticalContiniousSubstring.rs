impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut curr = 1;
        let mut ans = 1;

        for i in 1..bytes.len() {
            if bytes[i] == bytes[i - 1] + 1 {
                curr += 1;
            } else {
                curr = 1;
            }

            ans = ans.max(curr);
        }

        ans
    }
}
