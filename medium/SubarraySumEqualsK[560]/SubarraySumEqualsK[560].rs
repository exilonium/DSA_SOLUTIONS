//this is optimal
use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut res =0;
        let mut currSum = 0;
        let mut prefixSum: HashMap<i32, i32> = HashMap::with_capacity(nums.len() + 1);
        prefixSum.insert(0,1);
        
        for &i in nums.iter(){
            currSum+=i;
            
            // if let Some(&count) = prefixSum.get(&(currSum-k)){
            //     res+=count;
            // }
            res += prefixSum.get(&(currSum-k)).copied().unwrap_or(0);
            
            *prefixSum.entry(currSum).or_insert(0) += 1;
            // prefixSum.insert(currSum, 1+ prefixSum.get(&currSum).copied().unwrap_or(0));
            
        }
        return res
    }
}
