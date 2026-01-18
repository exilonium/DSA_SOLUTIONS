// clean code
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut res = vec![];
        if nums.is_empty() {
            return res;
        }
        let mut start = 0;
        let mut end = 1;
        let elem = |a, b| {
            if a == b {
                format!("{}", a)
            } else {
                format!("{}->{}", a, b)
            }
        };
        while end < nums.len() {
            if (nums[end - 1] + 1) != nums[end] {
                res.push(elem(nums[start], nums[end - 1]));
                start = end;
            }
            end += 1;
        }
        res.push(elem(nums[start], nums[end - 1]));
        res
    }
}
