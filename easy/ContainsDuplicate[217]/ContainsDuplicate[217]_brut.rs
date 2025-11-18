// time limit exceeded
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        for i in 0..nums.len()-1{
            for j in i+1..nums.len(){
                if nums[i]==nums[j]{
                    return true
                }
            }
        }
        return false
    
    }
}
