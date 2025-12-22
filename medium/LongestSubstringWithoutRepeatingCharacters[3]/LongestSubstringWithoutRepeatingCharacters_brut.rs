// kinda brut took 7 ms
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut count = 0;
        let mut myset = HashSet::new();
        while j < s.len() {
            if !myset.contains(&s[j..j + 1]) {
                myset.insert(&s[j..j + 1]);
                j += 1;
                count = std::cmp::max(count, j - i);
            } else {
                myset.remove(&s[i..i + 1]);
                i += 1;
            }
        }
        count as _
    }
}
