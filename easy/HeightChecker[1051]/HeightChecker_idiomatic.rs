// trying to learn idiomatic rust
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut sor = heights.clone();
        sor.sort_unstable();
        heights
            .iter()
            .zip(sor.iter())
            .filter(|(a, b)| a != b)
            .count() as i32
    }
}
