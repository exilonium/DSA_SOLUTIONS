// this is a brut force approach to get the Solution
// it takes 2ms and 2.44mb in leetcode
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<i32,i32> = HashMap::new();
        let mut count = 0;
        for i in nums {
            *map.entry(i).or_insert(0) += 1;
        }
        println!("{:?}",map);
        let mut max=i32::MIN;
        for (i,val) in map.iter_mut(){

            if *val>count{
                count= *val;
                max = *i;         
            }
        }
        return max
    }
}
