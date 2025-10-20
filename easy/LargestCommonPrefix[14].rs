use std::cmp::min;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut max_prefix = strs[0].to_string();
        let mut max_len = max_prefix.len();
        for (_i,value) in strs.iter().enumerate(){
            max_len = min(value.len(),max_len);
            while max_prefix[0..max_len]!=value[0..max_len]{
                max_len-=1;
            }
            max_prefix=max_prefix[0..max_len].to_string();

        }
        return max_prefix
    }
}
