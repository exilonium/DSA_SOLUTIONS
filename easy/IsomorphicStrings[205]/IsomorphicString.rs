// there are multilple solutions availiable for this problem
// this is one of the opimal and elegant solution i came up with
use std::collections::HashMap;
impl Solution {
    fn help(s: &[u8], t: &[u8]) -> bool {
        let mut map = HashMap::new();

        for (&a, &b) in s.iter().zip(t.iter()) {
            if *map.entry(a).or_insert(b) != b {
                return false;
            }
        }
        true
    }

    pub fn is_isomorphic(s: String, t: String) -> bool {
        let first = s.as_bytes();
        let second = t.as_bytes();
        Self::help(first, second) && Self::help(second, first)
    }
}
