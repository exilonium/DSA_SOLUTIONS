// better 16ms time
use std::collections::HashSet;
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut low = 1;
        let mut myset = HashSet::new();
        for i in nums {
            myset.insert(i);
        }
        while myset.contains(&low) {
            low += 1;
        }
        low
    }
}
