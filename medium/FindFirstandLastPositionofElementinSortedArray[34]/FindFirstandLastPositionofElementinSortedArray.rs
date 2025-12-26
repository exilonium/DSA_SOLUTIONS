impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left = Self::binary_search(&nums, target, true);
        let right = Self::binary_search(&nums, target, false);

        vec![left, right]
    }
    fn binary_search(nums: &Vec<i32>, target: i32, search_left: bool) -> i32 {
        let mut low: i32 = 0;
        let mut high: i32 = nums.len() as i32 - 1;
        let mut idx: i32 = -1;

        while low <= high {
            let mid = low + (high - low) / 2;
            let m = mid as usize;

            if nums[m] > target {
                high = mid - 1;
            } else if nums[m] < target {
                low = mid + 1;
            } else {
                idx = mid;
                if search_left {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            }
        }
        idx
    }
}
