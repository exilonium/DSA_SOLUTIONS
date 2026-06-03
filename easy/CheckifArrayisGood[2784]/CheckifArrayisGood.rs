impl Solution {
    pub fn is_good(mut nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return false;
        }
        nums.sort();
        let n = nums.len() as i32 - 1;
        for i in 0..n - 1 {
            if nums[i as usize] != i + 1 {
                return false;
            }
        }
        nums[(n - 1) as usize] == n && nums[n as usize] == n
    }
}
