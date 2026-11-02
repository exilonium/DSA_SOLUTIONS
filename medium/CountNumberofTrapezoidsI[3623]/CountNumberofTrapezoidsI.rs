use std::collections::HashMap;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut count = HashMap::new();

        for p in points {
            *count.entry(p[1]).or_insert(0_i64) += 1;
        }

        let mut ans = 0_i64;
        let mut sum = 0_i64;

        for &v in count.values() {
            let pairs = v * (v - 1) / 2;

            ans = (ans + sum * pairs) % MOD;
            sum += pairs;
        }

        ans as i32
    }
}
