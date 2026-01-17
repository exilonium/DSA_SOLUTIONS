impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut prev = b'a';
        for i in s.bytes() {
            if i < prev {
                return false;
            }
            prev = i;
        }
        return true;
    }
}
