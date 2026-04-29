impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut sum = 0;
        for i in 0..nums.len() {
            res += (i % nums.len()) as i32 * nums[i];
            sum += nums[i];
        }
        let mut tmp = res;
        for i in 0..nums.len() {
            tmp += sum - nums[nums.len() - i - 1] * nums.len() as i32;
            res = res.max(tmp);
        }
        res
    }
}
