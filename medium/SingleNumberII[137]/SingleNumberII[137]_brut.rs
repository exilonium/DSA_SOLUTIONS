// brut approach
use std::collections::HashMap;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut found = false;
        let mut map = HashMap::new();
        for i in &nums {
            map.entry(i)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        for (key, value) in map {
            match value {
                1 => return *key,
                _ => (),
            }
        }
        return -1;
    }
}

// this has 1 ms of time but this is not the optimal one pass approach
