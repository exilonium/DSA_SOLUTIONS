use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = vec![-1; nums1.len()];
        let mut map: HashMap<i32, usize> = nums1
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect::<HashMap<i32, usize>>();
        let mut monotonic: Vec<i32> = Vec::with_capacity(nums2.len());

        for n in nums2 {
            while monotonic.last().map_or(false, |last_num| n > *last_num) {
                monotonic.pop().map(|last_num| {
                    if let Some(index) = map.get(&last_num) {
                        result[*index] = n;
                    }
                });
            }
            monotonic.push(n);
        }

        result
    }
}
