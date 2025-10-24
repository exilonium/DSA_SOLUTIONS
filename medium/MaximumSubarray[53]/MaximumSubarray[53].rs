// this is kadane's algorithm

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        let mut sum =0;
        for i in &nums{
            sum+=i;
            max = std::cmp::max(sum,max); // this is comparing, like sum>max?sum:max in C
            if sum<0{
                sum=0;
            }
        }
        return max
    }
}
// todo 
// print the actual subarray
