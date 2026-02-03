// O(n) but still 0ms
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        if nums.len() == 2 {
            return if nums[0] > nums[1] { 0 } else { 1 };
        }
        let mut big = 0;
        for i in 1..nums.len() - 1 {
            if nums[i - 1] < nums[i] && nums[i + 1] < nums[i] {
                return i as _;
            }
        }
        for i in 0..nums.len() {
            if nums[i] > nums[big] {
                big = i;
            }
        }
        big as _
    }
}
