impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut curr_max = nums[0];
        for i in 1..nums.len() {
            let x = nums[i];
            res = res.max((curr_max - 1) * (x - 1));
            curr_max = curr_max.max(nums[i]);
        }
        res
    }
}
