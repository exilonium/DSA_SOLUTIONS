impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let k = (k1 as i64) + (k2 as i64);

        let diff: Vec<i64> = nums1
            .iter()
            .zip(nums2.iter())
            .map(|(&a, &b)| (a - b).abs() as i64)
            .collect();

        let total: i64 = diff.iter().sum();

        if total <= k {
            return 0;
        }

        let mut lo = 0i64;
        let mut hi = *diff.iter().max().unwrap() as i64;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            let needed: i64 = diff.iter().map(|&d| (d - mid).max(0)).sum();

            if needed <= k {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        let limit = lo;
        let mut remaining = k;
        let mut ans = 0i64;

        for &d in &diff {
            let reduced = d.min(limit);
            remaining -= d - reduced;
            ans += reduced * reduced;
        }

        // Any remaining operations reduce distinct values at the limit by 1.
        ans -= remaining * (2 * limit - 1);

        ans
    }
}
