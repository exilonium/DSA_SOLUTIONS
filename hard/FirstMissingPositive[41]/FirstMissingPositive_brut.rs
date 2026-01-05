// TLE
// this is a garbage code but still im putting it here
// brut
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut found = -1;
        for i in 1..i32::MAX {
            for j in &nums {
                if i == *j {
                    found = 1;
                    continue;
                }
            }
            if found == -1 {
                return i;
            }
            found = -1;
        }
        return -1;
    }
}
