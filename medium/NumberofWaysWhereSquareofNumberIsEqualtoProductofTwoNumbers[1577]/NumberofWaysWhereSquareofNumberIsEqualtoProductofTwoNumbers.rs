use std::collections::HashMap;

impl Solution {
    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        fn count(a: &[i32], b: &[i32]) -> i32 {
            let mut freq = HashMap::new();

            for &x in b {
                *freq.entry(x as i64).or_insert(0) += 1;
            }

            let mut ans = 0;

            for &x in a {
                let target = x as i64 * x as i64;

                for (&y, &fy) in &freq {
                    if y * y > target {
                        continue;
                    }

                    if y * y == target {
                        ans += fy * (fy - 1) / 2;
                    } else if target % y == 0 {
                        let z = target / y;

                        if z > y {
                            if let Some(&fz) = freq.get(&z) {
                                ans += fy * fz;
                            }
                        }
                    }
                }
            }

            ans
        }

        count(&nums1, &nums2) + count(&nums2, &nums1)
    }
}
