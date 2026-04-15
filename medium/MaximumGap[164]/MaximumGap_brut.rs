// kinda brut using inbuilt function
// 7 ms O(nlog(n)) time and O(1) space
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut diff = 0;
        for i in 0..(nums.len() - 1) {
            let d = nums[i + 1] - nums[i];
            diff = diff.max(d);
        }
        diff
    }
}
