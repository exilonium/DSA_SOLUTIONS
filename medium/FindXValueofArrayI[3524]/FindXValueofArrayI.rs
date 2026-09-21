impl Solution {
    pub fn result_array(mut a: Vec<i32>, k: i32) -> Vec<i64> {
        let k = k as usize;
        let mut res = vec![0i64; k];
        let mut freq = vec![0i64; k];

        for n in a.iter_mut() {
            *n %= k as i32;
            let n = *n as usize;

            let mut cur = vec![0i64; k];

            // Start a new subarray
            cur[n] = 1;

            // Extend previous subarrays
            for x in 0..k {
                cur[(x * n) % k] += freq[x];
            }

            // Update frequency and answer
            for x in 0..k {
                freq[x] = cur[x];
                res[x] += freq[x];
            }
        }

        res
    }
}
