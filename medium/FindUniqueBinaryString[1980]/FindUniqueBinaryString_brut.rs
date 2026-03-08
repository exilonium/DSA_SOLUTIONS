// 0ms O(n) time and space
use std::collections::HashSet;
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut n = nums[0].len() as u32; // looking at the len of the string (number of bits)
        let len = 2_i32.pow(n) - 1; // the range of integers the string can have 2^n -1
        let mut numbers = HashSet::new();
        for i in nums {
            let num = i32::from_str_radix(i.as_str(), 2).unwrap(); // converting
            // the binary string to number "10" -> 2
            numbers.insert(num); // storing all the numbers that are in the form of binary string
        }
        for i in 0..=len {
            if !numbers.contains(&i) {
                let res = format!("{:0width$b}", i, width = n as usize);
                // the width is there to  not have case like "0" when the expected ans is "0000"
                return res;
            }
        }
        " ".to_string() // just to satisfy the compiler
    }
}
