use std::collections::HashMap;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut freq = HashMap::new();

        for x in arr {
            *freq.entry(x).or_insert(0) += 1;
        }

        let mut buckets = vec![0; n + 1];

        for count in freq.values() {
            buckets[*count] += 1;
        }

        let mut removed = 0;
        let mut ans = 0;

        for i in (1..=n).rev() {
            while buckets[i] > 0 {
                removed += i;
                ans += 1;
                buckets[i] -= 1;

                if removed >= n / 2 {
                    return ans;
                }
            }
        }

        ans
    }
}
