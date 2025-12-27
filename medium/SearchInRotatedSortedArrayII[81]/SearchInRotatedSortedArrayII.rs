impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }
        let mut low = 0;
        let mut high = nums.len() - 1;
        while low <= high {
            let mid = low + (high - low) / 2;
            if nums[mid] == target {
                return true;
            }
            if nums[mid] == nums[low] && nums[mid] == nums[high] {
                low += 1;
                if high == 0 {
                    break;
                }
                high -= 1;
            } else if nums[low] <= nums[mid] {
                if nums[low] <= target && target < nums[mid] {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            } else {
                if nums[mid] < target && target <= nums[high] {
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
        }
        false
    }
}
