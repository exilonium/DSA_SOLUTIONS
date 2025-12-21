// optimal
// sadly i wasnt able to come up to this on my own :(
use std::collections::HashSet;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut all = HashSet::new(); // global distinct ORs
        let mut prev = HashSet::new(); // ORs ending at previous index

        for &num in &arr {
            let mut curr = HashSet::new();
            curr.insert(num);

            for &x in &prev {
                curr.insert(x | num);
            }

            for &v in &curr {
                all.insert(v);
            }

            prev = curr;
        }

        all.len() as i32
    }
}
