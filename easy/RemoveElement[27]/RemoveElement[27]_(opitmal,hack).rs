//brut (hack)
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // Remove all elements equal to val (kinda hacky way)
        nums.retain(|&x| x != val);

        // Return the new length
        nums.len() as i32
    }
}
