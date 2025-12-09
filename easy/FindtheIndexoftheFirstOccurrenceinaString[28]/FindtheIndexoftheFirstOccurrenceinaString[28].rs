//opimal leetcode way
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let h = haystack.as_bytes();
        let n = needle.as_bytes();

        let n_len = needle.len();

        if n_len == 0 {
            return 0;
        }
        if n_len > h.len() {
            return -1;
        }

        for i in 0..=(h.len() - n_len) {
            if &h[i..i + n_len] == n {
                return i as i32;
            }
        }
        return -1;
    }
}
