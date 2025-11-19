// optimal solution
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {

        let k = nums.len();
        let mut res = vec![0; k];
        let mut tmp =1;
        // making an subarray with the poduct subarray
        for i in 0..k{
            res[i] = tmp;
            tmp *= nums[i];
        }
        // multiplying that subarray to the postfix element to get the result
        tmp = 1;
        for i in (0..k).rev(){
            res[i] *= tmp;
            tmp *= nums[i];
        }
    res
    }
}
