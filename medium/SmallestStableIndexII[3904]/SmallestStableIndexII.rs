impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        // we make mins array: mins[i] = min(nums[i..n - 1])
        let mut mins = nums.clone();
        let mut curr_min = nums[nums.len() - 1];

        for i in (0..mins.len()).rev() {
            if mins[i] < curr_min {
                curr_min = mins[i];
            }
            mins[i] = curr_min;
        }

        //main aglo
        let mut curr_max = nums[0];
        for i in 0..nums.len() {
            if nums[i] > curr_max {
                curr_max = nums[i];
            }

            // check whether max(nums[0..i]) - min(nums[i..n - 1]) <= k
            //               curr_max        - mins[i]             <= k
            if curr_max - mins[i] <= k {
                return i as i32;
            }
        }
        -1
    }
}
