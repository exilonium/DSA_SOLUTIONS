// brut
use std::collections::HashSet;
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let mut found = HashSet::new();
        for i in &nums {
            found.insert(i);
        }
        for i in 1..=nums.len() {
            if !found.contains(&(i as i32)) {
                res.push(i as i32);
            }
        }
        res
    }
}
