// recusion + memoization O(n)
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let n = s.len();
        let bytes = s.as_bytes();

        // mem[i] = number of ways to decode substring starting at i
        let mut mem = vec![-1; n + 1];
        mem[n] = 1; // base case

        Self::dfs(0, bytes, &mut mem)
    }

    fn dfs(i: usize, s: &[u8], mem: &mut Vec<i32>) -> i32 {
        if mem[i] != -1 {
            return mem[i];
        }

        if s[i] == b'0' {
            mem[i] = 0;
            return 0;
        }

        let mut res = Self::dfs(i + 1, s, mem);

        if i + 1 < s.len() && (s[i] == b'1' || (s[i] == b'2' && s[i + 1] < b'7')) {
            res += Self::dfs(i + 2, s, mem);
        }

        mem[i] = res;
        res
    }
}
