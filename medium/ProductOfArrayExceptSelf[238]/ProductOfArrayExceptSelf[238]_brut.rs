// this is brut takes 5 ms on leetcode
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {

        let k = nums.len();
        let mut res = vec![0; k];
        let mut front = vec![0; k];
        let mut back  = vec![0; k];

        front[0]=nums[0];
        back[k-1]=nums[k-1];

        for i in 1..k{
            front[i]=nums[i]*front[i-1];
            back[k-1-i]=nums[k-1-i]*back[k-i];
        }

        res[0]=back[1];
        res[k-1]=front[k-2];

        for i in 1..k-1{
            res[i]=front[i-1]*back[i+1];
        }

        return res
    }
}
