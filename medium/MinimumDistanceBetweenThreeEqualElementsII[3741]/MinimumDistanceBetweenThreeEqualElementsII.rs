use std::collections::HashMap;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut positions: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut ans = i32::MAX;

        for (i, &x) in nums.iter().enumerate() {
            let pos = positions.entry(x).or_default();

            pos.push(i as i32);

            let n = pos.len();

            if n >= 3 {
                let a = pos[n - 3];
                let c = pos[n - 1];

                ans = ans.min(c - a);
            }
        }

        if ans == i32::MAX { -1 } else { ans * 2 }
    }
}
