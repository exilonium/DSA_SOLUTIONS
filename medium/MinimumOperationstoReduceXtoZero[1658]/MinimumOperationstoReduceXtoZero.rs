impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let target = nums.iter().sum::<i32>() - x;

        if target < 0 {
            return -1;
        }

        let mut left = 0;
        let mut sum = 0;
        let mut max_len = -1;

        for right in 0..nums.len() {
            sum += nums[right];

            while sum > target {
                sum -= nums[left];
                left += 1;
            }

            if sum == target {
                max_len = max_len.max(right as i32 - left as i32 + 1);
            }
        }

        if max_len == -1 {
            -1
        } else {
            nums.len() as i32 - max_len
        }
    }
}
