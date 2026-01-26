use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::new();

        for i in &nums1 {
            for j in &nums2 {
                if i == j {
                    set.insert(*i);
                }
            }
        }
        let res: Vec<i32> = set.into_iter().collect();
        res
    }
}
