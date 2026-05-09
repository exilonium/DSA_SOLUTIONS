// using xor better than addition since addition may have integer overflow
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut xor1: i32 = 0;
        let mut xor2: i32 = 0;
        for i in 0..=nums.len() {
            xor1 ^= i as i32;
        }
        for i in nums {
            xor2 ^= i;
        }
        xor1 ^ xor2
    }
}
