impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut lsum = vec![0; n];
        let mut rsum = vec![0; n];
        let mut res = vec![0; n];

        for i in 1..n {
            lsum[i] = lsum[i - 1] + nums[i - 1];
        }
        for i in (0..n - 1).rev() {
            rsum[i] = rsum[i + 1] + nums[i + 1];
        }
        for i in 0..n {
            res[i] = (lsum[i] - rsum[i]).abs();
        }
        res
    }
}
