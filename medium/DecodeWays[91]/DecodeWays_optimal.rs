//optimal
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }

        let bytes = s.as_bytes();
        let mut dp = vec![0; n + 1];

        // Empty suffix has one valid decoding
        dp[n] = 1;

        for i in (0..n).rev() {
            if bytes[i] != b'0' {
                // Decode single digit
                dp[i] = dp[i + 1];

                // Decode two digits if valid
                if i + 1 < n && (bytes[i] == b'1' || (bytes[i] == b'2' && bytes[i + 1] < b'7')) {
                    dp[i] += dp[i + 2];
                }
            }
        }

        dp[0]
    }
}
