// single pass method using index knowledge
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut res: i32 = nums.len() as i32;
        for i in 0..nums.len() {
            res += i as i32 - nums[i];
        }
        res
    }
}
