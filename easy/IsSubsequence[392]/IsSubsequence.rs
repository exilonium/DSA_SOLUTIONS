impl Solution {
    pub fn is_subsequence(s1: String, s2: String) -> bool {
        let (s1, s2, mut p1, mut p2) = (s1.as_bytes(), s2.as_bytes(), 0, 0);
        while p1 < s1.len() && p2 < s2.len() {
            if s1[p1] == s2[p2] {
                p1 += 1;
            }
            p2 += 1;
        }

        return p1 == s1.len();
    }
}
