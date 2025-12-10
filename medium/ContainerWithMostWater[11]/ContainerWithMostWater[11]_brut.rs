// using brut approach O(n^2)
// this works but time limit exceeded on a specific testcase
use std::cmp::max;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..height.len() {
            let mut vol = 0;
            for j in i..height.len() {
                let width: i32 = (j - i) as i32;
                if height[i] < height[j] {
                    vol = height[i] * width;
                } else {
                    vol = height[j] * width;
                }
                res = max(res, vol);
            }
        }
        res
    }
}
