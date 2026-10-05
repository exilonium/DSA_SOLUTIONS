impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let score = nums[0] + nums[1];
        let mut ans = 0;

        for i in (0..nums.len() - 1).step_by(2) {
            if nums[i] + nums[i + 1] != score {
                break;
            }

            ans += 1;
        }

        ans
    }
}
