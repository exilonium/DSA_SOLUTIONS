// brut
// TLE 67/87 cases passed
use std::collections::HashMap;
impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for i in 0..arr.len() {
            for j in i..arr.len() {
                let mut or = 0;
                for k in i..j + 1 {
                    or |= arr[k];
                    map.entry(or)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                }
            }
        }
        map.len() as _
    }
}
