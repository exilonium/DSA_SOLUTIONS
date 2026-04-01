use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = HashMap::new();
        for i in s.chars() {
            map.entry(i).and_modify(|count| *count += 1).or_insert(1);
        }
        for (idx, ch) in s.chars().enumerate() {
            if *map.get(&ch).unwrap() == 1 {
                return idx as _;
            }
        }
        -1
    }
}
