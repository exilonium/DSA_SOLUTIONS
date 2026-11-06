use std::collections::HashMap;

impl Solution {
    pub fn count_servers(n: i32, logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        let mut logs = logs;
        let mut queries_with_idx: Vec<(i32, usize)> =
            queries.iter().enumerate().map(|(i, &q)| (q, i)).collect();

        logs.sort_unstable_by_key(|log| log[1]);
        queries_with_idx.sort_unstable();

        let mut count = HashMap::new();
        let mut left = 0;
        let mut right = 0;
        let mut ans = vec![0; queries.len()];

        for (t, idx) in queries_with_idx {
            // Add requests with time <= t
            while right < logs.len() && logs[right][1] <= t {
                *count.entry(logs[right][0]).or_insert(0) += 1;
                right += 1;
            }

            // Remove requests with time < t - x
            while left < right && logs[left][1] < t - x {
                let server = logs[left][0];

                if let Some(c) = count.get_mut(&server) {
                    *c -= 1;

                    if *c == 0 {
                        count.remove(&server);
                    }
                }

                left += 1;
            }

            ans[idx] = n - count.len() as i32;
        }

        ans
    }
}
