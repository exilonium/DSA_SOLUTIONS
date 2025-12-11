// optimal
use std::cmp::max;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = height.len() - 1;

        let mut vol = 0;
        let mut maxvol = 0;
        while low < high {
            let l = height[low];
            let r = height[high];
            if l < r {
                vol = l * ((high - low) as i32);
                low += 1;
            } else {
                vol = r * ((high - low) as i32);
                high -= 1;
            }
            maxvol = max(maxvol, vol);
        }
        maxvol
    }
}
