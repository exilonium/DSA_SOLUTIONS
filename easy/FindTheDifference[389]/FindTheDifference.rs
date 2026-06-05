impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut res = 0;

        s.as_bytes().iter().for_each(|c| res ^= c);
        t.as_bytes().iter().for_each(|c| res ^= c);

        res as char
    }
}
