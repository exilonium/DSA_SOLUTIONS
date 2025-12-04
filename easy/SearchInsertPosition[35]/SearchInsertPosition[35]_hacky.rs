// one liner hacky idiomatic rust approach
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(i) | Err(i) => i as i32, // binary_search returns the index if found
                                        // or the index where it should be inserted
        }
    }
}
