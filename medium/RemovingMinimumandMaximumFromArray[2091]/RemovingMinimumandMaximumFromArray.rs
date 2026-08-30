impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut min_idx = 0;
        let mut max_idx = 0;

        for i in 1..n {
            if nums[i] < nums[min_idx] {
                min_idx = i;
            }
            if nums[i] > nums[max_idx] {
                max_idx = i;
            }
        }

        // Put min/max indices in order
        let left = min_idx.min(max_idx);
        let right = min_idx.max(max_idx);

        let remove_left = right + 1;

        let remove_right = n - left;

        // 3. Remove left element from left, right element from right
        let remove_both = (left + 1) + (n - right);

        remove_left.min(remove_right).min(remove_both) as i32
    }
}
