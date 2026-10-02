impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut ans = 0;

        for c in (2..nums.len()).rev() {
            let mut left = 0;
            let mut right = c - 1;

            while left < right {
                if nums[left] + nums[right] > nums[c] {
                    ans += (right - left) as i32;
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }

        ans
    }
}
