impl Solution {
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        while nums.len() > 1 {
            let mut next = Vec::with_capacity(nums.len() / 2);

            for i in 0..nums.len() / 2 {
                if i % 2 == 0 {
                    next.push(nums[2 * i].min(nums[2 * i + 1]));
                } else {
                    next.push(nums[2 * i].max(nums[2 * i + 1]));
                }
            }

            nums = next;
        }

        nums[0]
    }
}
