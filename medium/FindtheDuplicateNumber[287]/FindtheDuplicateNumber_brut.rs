// brut
use std::collections::HashSet;
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for i in nums {
            if set.contains(&i) {
                return i;
            }
            set.insert(i);
        }
        return -1;
    }
}
