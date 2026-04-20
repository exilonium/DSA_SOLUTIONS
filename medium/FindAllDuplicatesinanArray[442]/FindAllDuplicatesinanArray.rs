impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let size = nums.len();

        let mut res = Vec::new();

        for i in 0..size {
            let pos = (nums[i].abs() - 1) as usize;
            if nums[pos] < 0 {
                res.push(pos as i32 + 1);
            }
            nums[pos] = -nums[pos];
        }
        res
    }
}
