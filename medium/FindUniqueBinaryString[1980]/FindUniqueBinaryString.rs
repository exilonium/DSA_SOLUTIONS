// cantor diagonal theorem
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut result: String = String::new();

        for i in 0..nums.len() {
            let s_bytes = nums[i].as_bytes();
            let mut k: char = if s_bytes[i] == b'1' { '0' } else { '1' };
            result = format!("{}{}", result, k).to_string();
        }

        result
    }
}
