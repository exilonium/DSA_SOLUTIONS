// binary search
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = nums.len() - 1;
        while low < high {
            let mid = (low + high) / 2;
            if nums[mid] > nums[mid + 1] {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low as _
    }
}
