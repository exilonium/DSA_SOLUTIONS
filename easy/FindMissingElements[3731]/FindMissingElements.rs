// using bool array instead of -1
impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut state = vec![false; 101];
        let mut max = 0;
        let mut min = 101;
        for &i in &nums {
            state[i as usize] = true;
            max = max.max(i);
            min = min.min(i);
        }
        let mut res = Vec::new();
        for i in min..max {
            if !state[i as usize] {
                res.push(i);
            }
        }
        res
    }
}
