use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<i32> = nums1.into_iter().collect();
        nums2
            .into_iter() // iterate and takes value
            .filter(|x| set1.contains(x)) // filters the value if present in set1
            .collect::<HashSet<i32>>() // collect the values to the set (to dedupe)
            .into_iter()
            .collect() //collect the unique values
    }
}
