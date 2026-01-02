// 5ms Solution
use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let mut freq: HashMap<u8, i32> = HashMap::new();

        let mut left = 0;
        let mut max_freq = 0;
        let mut result = 0;

        for right in 0..bytes.len() {
            let c = bytes[right];
            *freq.entry(c).or_insert(0) += 1;
            // same as map.entry(or).and_modify(|counter| *counter += 1).or_insert(1);
            max_freq = max_freq.max(freq[&c]);

            while (right - left + 1) as i32 > max_freq + k {
                let left_char = bytes[left];
                *freq.get_mut(&left_char).unwrap() -= 1;
                left += 1;
            }
        }
        result = s.len() - left;

        result as _
    }
}
