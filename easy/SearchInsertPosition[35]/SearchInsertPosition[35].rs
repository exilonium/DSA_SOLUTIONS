//optimal
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len();
        while low < high {
            let mid = (low + high) / 2;
            if nums[mid] < target {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        return low as i32;
    }
}
