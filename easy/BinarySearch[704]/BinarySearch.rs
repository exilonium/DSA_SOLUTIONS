impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut high: i32 = nums.len() as i32 - 1;
        let mut low: i32 = 0;
        while low <= high {
            let mid = low + (high - low) / 2;
            println!("here");
            if nums[mid as usize] == target {
                return mid;
            } else if nums[mid as usize] > target {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        return -1;
    }
}
