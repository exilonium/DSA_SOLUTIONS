//using hash set 0ms time and 3.76mb space
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();
        for i in nums{
            if !set.insert(i){
                return true;
            }
        }
        false
    }
}
