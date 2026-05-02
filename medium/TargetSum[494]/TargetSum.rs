use std::collections::HashMap;
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let freq = if nums[0] == 0 {
            HashMap::from([(0, 2)])
        } else {
            HashMap::from([(nums[0], 1), (-nums[0], 1)])
        };
        // above was just building a base freqency table
        let freq = nums.into_iter().skip(1).fold(freq, |f, n| {
            let mut tmp = HashMap::new();
            f.into_iter().for_each(|(k, v)| {
                let v1 = k + n;
                let v2 = k - n;
                tmp.entry(v1).and_modify(|c| *c += v).or_insert(v);
                tmp.entry(v2).and_modify(|c| *c += v).or_insert(v);
            });
            tmp
        });
        *freq.get(&target).unwrap_or(&0)
    }
}
