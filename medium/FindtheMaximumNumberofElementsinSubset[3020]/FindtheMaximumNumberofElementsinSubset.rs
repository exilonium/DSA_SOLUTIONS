use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut mp = HashMap::new();

        for &x in &nums {
            *mp.entry(x as i64).or_insert(0) += 1;
        }

        let mut ans = 1;

        for &x in &nums {
            let x = x as i64;

            if x == 1 {
                let len = if mp[&x] % 2 == 1 { mp[&x] } else { mp[&x] - 1 };

                ans = ans.max(len);
            } else {
                let mut ct = 0;

                if mp[&x] >= 2 {
                    let mut curr = x;

                    while curr <= i32::MAX as i64 && mp.contains_key(&curr) {
                        if mp[&curr] == 1 {
                            ct += 1;
                            break;
                        }

                        ct += 1;

                        // Prevent overflow
                        if curr > i64::MAX / curr {
                            break;
                        }

                        curr *= curr;
                    }
                }

                ans = ans.max(ct * 2 - 1);
            }
        }

        ans
    }
}
