impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut top = 0;
        let mut curr = 0;
        for i in gain {
            curr += i;
            top = curr.max(top);
        }
        top
    }
}
