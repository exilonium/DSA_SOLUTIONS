// pure recursion O(2^n)
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let bytes = s.as_bytes();
        Self::dfs(0, bytes)
    }

    fn dfs(i: usize, s: &[u8]) -> i32 {
        let n = s.len();

        // Reached end -> one valid decoding
        if i == n {
            return 1;
        }

        // Leading zero is invalid
        if s[i] == b'0' {
            return 0;
        }

        // Decode single character
        let mut res = Self::dfs(i + 1, s);

        // Decode two characters if valid (10–26)
        if i + 1 < n && (s[i] == b'1' || (s[i] == b'2' && s[i + 1] < b'7')) {
            res += Self::dfs(i + 2, s);
        }

        res
    }
}
