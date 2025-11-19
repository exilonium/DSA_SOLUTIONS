// optimal 
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut map:HashMap<i32,i32>= HashMap::new();
        // storing elements in the hashmap with their frequency
        for i in &nums{
            map.entry(*i).and_modify(|counter| *counter+=1).or_insert(1);
        }

        // using bucket sort to store the frequency and the number 
        // example [[], [], [3], [], [2, 1], [], [], [], [], [], []]
        let mut freq:Vec<Vec<i32>>= vec![Vec::new(); nums.len() + 1];
        for (i,j) in &map{
            freq[*j as usize].push(*i);
        }

        // storing the result in the new vector since the most frequency is in the last..
        let mut res :Vec<i32>= Vec::new();
        for i in freq.iter().rev(){
            for j in i.iter(){
                res.push(*j);
                if res.len()==k{
                    return res
                }
            }
       }
       return res
    }
}
