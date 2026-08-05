// not my solution
use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut max_x = -1;

        let mut freq: HashMap<i32, usize> = HashMap::new();
        for &x in arr.iter() {
            *freq.entry(x).or_insert(0) += 1;
        }
        for x in arr.iter() {
            match freq.get(x) {
                Some(&f) => {
                    if *x as usize == f {
                        max_x = max(max_x, f as i32);
                    }
                }
                _ => continue,
            }
        }

        max_x
    }
}
