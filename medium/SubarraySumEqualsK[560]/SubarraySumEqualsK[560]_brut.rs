//this is brut
// 1006ms  in time, beats 7.19% and 2.38 mb in memory, beats 97.39%
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut total_sub = 0;
        for i in 0..nums.len(){
            let mut sum =0;
            for j in i..nums.len(){
                sum+=nums[j];
                if sum==k{
                    total_sub+=1;
                }
            }
        }
       total_sub
    }
}
