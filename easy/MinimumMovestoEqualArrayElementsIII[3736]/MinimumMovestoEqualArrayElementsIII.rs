impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut sum = 0;

        for &i in &nums {
            sum += i;
            max = max.max(i);
        }

        (nums.len() as i32 * max) - sum
    }
}
