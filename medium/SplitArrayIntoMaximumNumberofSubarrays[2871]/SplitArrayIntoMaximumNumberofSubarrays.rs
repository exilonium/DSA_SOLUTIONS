impl Solution {
    pub fn max_subarrays(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut curr = -1;

        for num in nums {
            curr &= num;

            if curr == 0 {
                ans += 1;
                curr = -1;
            }
        }

        ans.max(1)
    }
}
